//! Public, unauthenticated Roblox API lookups (games, thumbnails, users).
//!
//! All endpoints here are public and require no auth. We only call them when
//! the detected place changes, so traffic is minimal.

use serde_json::Value;
use std::sync::OnceLock;
use std::time::Duration;
use ureq::Agent;

#[derive(Debug, Clone, Default)]
pub struct GameInfo {
    pub universe_id: u64,
    pub name: String,
    pub creator_name: String,
    pub icon_url: Option<String>,
    pub playing: Option<u64>,
    pub max_players: Option<u64>,
}

fn agent() -> &'static Agent {
    static AGENT: OnceLock<Agent> = OnceLock::new();
    AGENT.get_or_init(|| {
        ureq::AgentBuilder::new()
            .timeout_connect(Duration::from_secs(6))
            .timeout_read(Duration::from_secs(8))
            .user_agent("RoPresence/0.1 (+https://github.com/ropresence)")
            .build()
    })
}

fn get_json(url: &str) -> Option<Value> {
    agent().get(url).call().ok()?.into_json().ok()
}

/// place id -> universe id.
pub fn resolve_universe(place_id: u64) -> Option<u64> {
    let url = format!("https://apis.roblox.com/universes/v1/places/{place_id}/universe");
    get_json(&url)?.get("universeId")?.as_u64()
}

/// universe id -> (name, creator name, playing, max players).
pub fn fetch_details(universe_id: u64) -> Option<(String, String, Option<u64>, Option<u64>)> {
    let url = format!("https://games.roblox.com/v1/games?universeIds={universe_id}");
    let v = get_json(&url)?;
    let first = v.get("data")?.as_array()?.first()?;
    let name = first.get("name")?.as_str()?.to_string();
    let creator = first
        .get("creator")
        .and_then(|c| c.get("name"))
        .and_then(|n| n.as_str())
        .unwrap_or("")
        .to_string();
    let playing = first.get("playing").and_then(|p| p.as_u64());
    let max_players = first.get("maxPlayers").and_then(|p| p.as_u64());
    Some((name, creator, playing, max_players))
}

/// universe id -> icon image URL (512px PNG).
pub fn fetch_icon(universe_id: u64) -> Option<String> {
    let url = format!(
        "https://thumbnails.roblox.com/v1/games/icons?universeIds={universe_id}&size=512x512&format=Png&isCircular=false"
    );
    let v = get_json(&url)?;
    let first = v.get("data")?.as_array()?.first()?;
    if first.get("state").and_then(|s| s.as_str()) == Some("Completed") {
        first
            .get("imageUrl")
            .and_then(|u| u.as_str())
            .map(String::from)
    } else {
        None
    }
}

/// Convenience: resolve everything for a place in one call.
pub fn fetch_game_info(place_id: u64, known_universe: Option<u64>) -> Option<GameInfo> {
    let universe_id = match known_universe {
        Some(u) if u > 0 => u,
        _ => resolve_universe(place_id)?,
    };
    let (name, creator_name, playing, max_players) =
        fetch_details(universe_id).unwrap_or_default();
    let icon_url = fetch_icon(universe_id);
    Some(GameInfo {
        universe_id,
        name,
        creator_name,
        icon_url,
        playing,
        max_players,
    })
}

/// username -> user id (used for avatar thumbnails, LOT 2 #32).
pub fn resolve_user_id(username: &str) -> Option<u64> {
    let body = serde_json::json!({ "usernames": [username], "excludeBannedUsers": false });
    let resp: Value = agent()
        .post("https://users.roblox.com/v1/usernames/users")
        .send_json(body)
        .ok()?
        .into_json()
        .ok()?;
    resp.get("data")?.as_array()?.first()?.get("id")?.as_u64()
}

/// user id -> avatar headshot URL.
pub fn fetch_avatar(user_id: u64) -> Option<String> {
    let url = format!(
        "https://thumbnails.roblox.com/v1/users/avatar-headshot?userIds={user_id}&size=420x420&format=Png&isCircular=false"
    );
    let v = get_json(&url)?;
    let first = v.get("data")?.as_array()?.first()?;
    first
        .get("imageUrl")
        .and_then(|u| u.as_str())
        .map(String::from)
}
