use regex::Regex;
use std::fs::{self, File};
use std::io::{Read, Seek, SeekFrom};
use std::path::PathBuf;
use std::sync::OnceLock;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ParsedLog {
    pub place_id: Option<u64>,
    pub universe_id: Option<u64>,
    pub job_id: Option<String>,
    pub in_game: bool,
    pub join_elapsed_secs: Option<i64>,
}

fn line_secs(line: &str) -> Option<f64> {
    line.splitn(3, ',').nth(1)?.trim().parse::<f64>().ok()
}

fn re_join() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"Joining game '([0-9a-fA-F\-]{8,})' place (\d+)").unwrap())
}

fn re_universe() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"(?i)universeid[:=]?\s*(\d+)").unwrap())
}

fn re_place() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"(?i)placeid[:=]?\s*(\d+)").unwrap())
}

// Signaux de départ FIABLES uniquement : retour au menu / fermeture du jeu.
// On exclut volontairement les Disconnect réseau génériques (Client:Disconnect,
// "Disconnect from") car Roblox les émet aussi lors d'un téléport, juste APRÈS
// la nouvelle ligne "Joining game" — ce qui faisait croire à tort à un départ
// (jeu affiché comme "menu" alors qu'on est en jeu).
fn re_leave() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(
            r"(?i)(handleGameWillClose|leaveUGCGame|done disconnecting|GameDisconnect)",
        )
        .unwrap()
    })
}

fn log_dir() -> Option<PathBuf> {
    let local = std::env::var_os("LOCALAPPDATA")?;
    let dir = PathBuf::from(local).join("Roblox").join("logs");
    if dir.is_dir() {
        Some(dir)
    } else {
        None
    }
}

fn latest_log(dir: &PathBuf) -> Option<PathBuf> {
    let mut newest: Option<(std::time::SystemTime, PathBuf)> = None;
    for entry in fs::read_dir(dir).ok()?.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("log") {
            continue;
        }
        let modified = match entry.metadata().and_then(|m| m.modified()) {
            Ok(t) => t,
            Err(_) => continue,
        };
        if newest.as_ref().map(|(t, _)| modified > *t).unwrap_or(true) {
            newest = Some((modified, path));
        }
    }
    newest.map(|(_, p)| p)
}

fn read_tail(path: &PathBuf, max: u64) -> std::io::Result<String> {
    let mut file = File::open(path)?;
    let len = file.metadata()?.len();
    let start = len.saturating_sub(max);
    file.seek(SeekFrom::Start(start))?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;
    Ok(String::from_utf8_lossy(&bytes).into_owned())
}

pub fn parse_latest() -> Option<ParsedLog> {
    let dir = log_dir()?;
    let file = latest_log(&dir)?;
    let content = read_tail(&file, 256 * 1024).ok()?;
    Some(parse_content(&content))
}

pub fn parse_content(content: &str) -> ParsedLog {
    let mut result = ParsedLog::default();
    let mut join_line: Option<usize> = None;
    let mut leave_line: Option<usize> = None;
    let mut join_secs: Option<f64> = None;
    let mut last_secs: Option<f64> = None;

    for (idx, line) in content.lines().enumerate() {
        if let Some(s) = line_secs(line) {
            last_secs = Some(s);
        }
        if let Some(caps) = re_join().captures(line) {
            result.job_id = caps.get(1).map(|m| m.as_str().to_string());
            result.place_id = caps.get(2).and_then(|m| m.as_str().parse().ok());
            join_line = Some(idx);
            join_secs = line_secs(line);
        }
        if let Some(caps) = re_universe().captures(line) {
            if let Some(id) = caps.get(1).and_then(|m| m.as_str().parse::<u64>().ok()) {
                if id > 0 {
                    result.universe_id = Some(id);
                }
            }
        }
        if result.place_id.is_none() {
            if let Some(caps) = re_place().captures(line) {
                result.place_id = caps.get(1).and_then(|m| m.as_str().parse().ok());
            }
        }
        if re_leave().is_match(line) {
            leave_line = Some(idx);
        }
    }

    result.in_game = match join_line {
        Some(j) => leave_line.map(|l| l < j).unwrap_or(true),
        None => false,
    };

    if result.in_game {
        if let (Some(j), Some(l)) = (join_secs, last_secs) {
            let elapsed = (l - j).max(0.0);
            if elapsed < 86_400.0 {
                result.join_elapsed_secs = Some(elapsed as i64);
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_join() {
        let log = "[FLog::Output] ! Joining game 'abcd1234-aaaa-bbbb-cccc-1234567890ab' place 920587237 at 1.2.3.4\n[FLog::GameJoinLoadTime] universeId:245662005, placeId:920587237";
        let parsed = parse_content(log);
        assert_eq!(parsed.place_id, Some(920587237));
        assert_eq!(parsed.universe_id, Some(245662005));
        assert!(parsed.in_game);
    }

    #[test]
    fn detects_leave_after_join() {
        let log = "Joining game 'abcdef12-3456-7890' place 111 at 1.2.3.4\n[FLog::SingleSurfaceApp] leaveUGCGameInternal";
        let parsed = parse_content(log);
        assert_eq!(parsed.place_id, Some(111));
        assert!(!parsed.in_game);
    }

    // Régression : un téléport émet "Joining game" puis un Client:Disconnect de
    // l'ancienne connexion juste après. On doit rester EN JEU, pas dans le menu.
    #[test]
    fn teleport_disconnect_after_join_stays_in_game() {
        let log = "2026-06-18T18:05:39.390Z,17332.39,56f0,6 [FLog::Output] ! Joining game '90115922-731c-4c3c-978a-9caf51bc5732' place 12699642568 at 10.206.16.94\n\
2026-06-18T18:05:40.066Z,17333.06,5df8,6,Info [DFLog::NetworkClient] Client:Disconnect\n\
2026-06-18T18:05:40.765Z,17333.76,5df8,6,Info [DFLog::NetworkClient] Client:Disconnect";
        let parsed = parse_content(log);
        assert_eq!(parsed.place_id, Some(12699642568));
        assert!(parsed.in_game, "un Disconnect réseau post-téléport ne doit pas être traité comme un départ");
    }
}
