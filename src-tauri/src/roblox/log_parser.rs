//! Parse the local Roblox client logs to find the current place/job/universe.
//!
//! Roblox writes plain-text logs to `%LOCALAPPDATA%\Roblox\logs\*.log`. We read
//! only the tail of the most recent file and look for join/leave markers. This
//! is read-only and never authenticates as the user.

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
    /// True when the latest join marker has no later leave marker.
    pub in_game: bool,
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

fn re_leave() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(
            r"(?i)(Client:Disconnect|handleGameWillClose|leaveUGCGame|done disconnecting|Disconnect from|GameDisconnect)",
        )
        .unwrap()
    })
}

/// Resolve `%LOCALAPPDATA%\Roblox\logs`.
fn log_dir() -> Option<PathBuf> {
    let local = std::env::var_os("LOCALAPPDATA")?;
    let dir = PathBuf::from(local).join("Roblox").join("logs");
    if dir.is_dir() {
        Some(dir)
    } else {
        None
    }
}

/// Most recently modified `.log` file in the logs directory.
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

/// Read up to `max` bytes from the end of a file (UTF-8 lossy).
fn read_tail(path: &PathBuf, max: u64) -> std::io::Result<String> {
    let mut file = File::open(path)?;
    let len = file.metadata()?.len();
    let start = len.saturating_sub(max);
    file.seek(SeekFrom::Start(start))?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;
    Ok(String::from_utf8_lossy(&bytes).into_owned())
}

/// Parse the tail of the newest log file. Returns `None` if no log exists.
pub fn parse_latest() -> Option<ParsedLog> {
    let dir = log_dir()?;
    let file = latest_log(&dir)?;
    let content = read_tail(&file, 256 * 1024).ok()?;
    Some(parse_content(&content))
}

/// Pure parsing of log text — split out for testability.
pub fn parse_content(content: &str) -> ParsedLog {
    let mut result = ParsedLog::default();
    let mut join_line: Option<usize> = None;
    let mut leave_line: Option<usize> = None;

    for (idx, line) in content.lines().enumerate() {
        if let Some(caps) = re_join().captures(line) {
            result.job_id = caps.get(1).map(|m| m.as_str().to_string());
            result.place_id = caps.get(2).and_then(|m| m.as_str().parse().ok());
            join_line = Some(idx);
        }
        if let Some(caps) = re_universe().captures(line) {
            if let Some(id) = caps.get(1).and_then(|m| m.as_str().parse::<u64>().ok()) {
                if id > 0 {
                    result.universe_id = Some(id);
                }
            }
        }
        // Fall back to a generic placeId marker if no explicit join line yet.
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
        let log = "Joining game 'id-1234' place 111 at 1.2.3.4\n[FLog::Network] Client:Disconnect";
        let parsed = parse_content(log);
        assert_eq!(parsed.place_id, Some(111));
        assert!(!parsed.in_game);
    }
}
