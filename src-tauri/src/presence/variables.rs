#[derive(Debug, Clone, Default)]
pub struct VarContext {
    pub game: String,
    pub creator: String,
    pub username: String,
    pub user_id: String,
    pub place_id: String,
    pub universe_id: String,
    pub players: String,
    pub max_players: String,
    pub job_id: String,
    pub date: String,
    pub status: String,
    pub elapsed_secs: Option<i64>,
    pub game_elapsed_secs: Option<i64>,
    pub daily_secs: Option<i64>,
}

fn format_elapsed(secs: i64) -> String {
    let secs = secs.max(0);
    let h = secs / 3600;
    let m = (secs % 3600) / 60;
    if h > 0 {
        format!("{h}h {m:02}m")
    } else {
        format!("{m}m")
    }
}

fn server_size(ctx: &VarContext) -> String {
    if ctx.players.is_empty() || ctx.max_players.is_empty() {
        String::new()
    } else {
        format!("{}/{}", ctx.players, ctx.max_players)
    }
}

fn fill_pct(ctx: &VarContext) -> String {
    let cur = ctx.players.parse::<f64>().ok();
    let max = ctx.max_players.parse::<f64>().ok();
    match (cur, max) {
        (Some(c), Some(m)) if m > 0.0 => format!("{}%", ((c / m) * 100.0).round() as i64),
        _ => String::new(),
    }
}

fn job_short(ctx: &VarContext) -> String {
    ctx.job_id.chars().take(8).collect()
}

pub fn resolve(template: &str, ctx: &VarContext) -> String {
    let session_time = ctx.elapsed_secs.map(format_elapsed).unwrap_or_default();
    let game_time = ctx.game_elapsed_secs.map(format_elapsed).unwrap_or_default();
    let daily = ctx.daily_secs.map(format_elapsed).unwrap_or_default();

    template
        // ordre important : les variables longues d'abord pour éviter
        // qu'un préfixe plus court ne soit substitué en premier.
        .replace("{maxPlayers}", &ctx.max_players)
        .replace("{serverSize}", &server_size(ctx))
        .replace("{sessionTime}", &session_time)
        .replace("{gameTime}", &game_time)
        .replace("{universeId}", &ctx.universe_id)
        .replace("{creatorUpper}", &ctx.creator.to_uppercase())
        .replace("{gameUpper}", &ctx.game.to_uppercase())
        .replace("{jobShort}", &job_short(ctx))
        .replace("{placeId}", &ctx.place_id)
        .replace("{username}", &ctx.username)
        .replace("{creator}", &ctx.creator)
        .replace("{players}", &ctx.players)
        .replace("{status}", &ctx.status)
        .replace("{userId}", &ctx.user_id)
        .replace("{jobId}", &ctx.job_id)
        .replace("{daily}", &daily)
        .replace("{game}", &ctx.game)
        .replace("{date}", &ctx.date)
        .replace("{fill}", &fill_pct(ctx))
        .replace("{time}", &session_time)
        .trim()
        .to_string()
}

pub const SUPPORTED_VARIABLES: &[&str] = &[
    "{game}",
    "{creator}",
    "{username}",
    "{userId}",
    "{placeId}",
    "{universeId}",
    "{players}",
    "{maxPlayers}",
    "{serverSize}",
    "{fill}",
    "{jobId}",
    "{jobShort}",
    "{time}",
    "{sessionTime}",
    "{gameTime}",
    "{daily}",
    "{date}",
    "{status}",
    "{gameUpper}",
    "{creatorUpper}",
];

#[cfg(test)]
mod tests {
    use super::*;

    fn ctx() -> VarContext {
        VarContext {
            game: "Adopt Me".into(),
            creator: "Uplift".into(),
            username: "noa".into(),
            user_id: "123".into(),
            place_id: "920587237".into(),
            universe_id: "245662005".into(),
            players: "12".into(),
            max_players: "30".into(),
            job_id: "90115922-731c-4c3c".into(),
            date: "2026-06-18".into(),
            status: "En jeu".into(),
            elapsed_secs: Some(3725),
            game_elapsed_secs: Some(600),
            daily_secs: Some(7200),
        }
    }

    #[test]
    fn resolves_all_variables() {
        let c = ctx();
        assert_eq!(resolve("{game}", &c), "Adopt Me");
        assert_eq!(resolve("{gameUpper}", &c), "ADOPT ME");
        assert_eq!(resolve("{creatorUpper}", &c), "UPLIFT");
        assert_eq!(resolve("{maxPlayers}", &c), "30");
        assert_eq!(resolve("{serverSize}", &c), "12/30");
        assert_eq!(resolve("{fill}", &c), "40%");
        assert_eq!(resolve("{jobShort}", &c), "90115922");
        assert_eq!(resolve("{time}", &c), "1h 02m");
        assert_eq!(resolve("{sessionTime}", &c), "1h 02m");
        assert_eq!(resolve("{gameTime}", &c), "10m");
        assert_eq!(resolve("{daily}", &c), "2h 00m");
        assert_eq!(resolve("{date}", &c), "2026-06-18");
        assert_eq!(resolve("{status}", &c), "En jeu");
        // combinaison
        assert_eq!(resolve("{game} · {serverSize}", &c), "Adopt Me · 12/30");
    }

    #[test]
    fn empty_when_missing_data() {
        let c = VarContext::default();
        assert_eq!(resolve("{serverSize}", &c), "");
        assert_eq!(resolve("{fill}", &c), "");
        assert_eq!(resolve("{maxPlayers}", &c), "");
    }
}
