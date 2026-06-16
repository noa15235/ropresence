#[derive(Debug, Clone, Default)]
pub struct VarContext {
    pub game: String,
    pub creator: String,
    pub username: String,
    pub user_id: String,
    pub place_id: String,
    pub universe_id: String,
    pub players: String,
    pub job_id: String,
    pub elapsed_secs: Option<i64>,
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

pub fn resolve(template: &str, ctx: &VarContext) -> String {
    let time = ctx
        .elapsed_secs
        .map(format_elapsed)
        .unwrap_or_default();

    template
        .replace("{game}", &ctx.game)
        .replace("{creator}", &ctx.creator)
        .replace("{username}", &ctx.username)
        .replace("{userId}", &ctx.user_id)
        .replace("{placeId}", &ctx.place_id)
        .replace("{universeId}", &ctx.universe_id)
        .replace("{players}", &ctx.players)
        .replace("{jobId}", &ctx.job_id)
        .replace("{time}", &time)
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
    "{jobId}",
    "{time}",
];
