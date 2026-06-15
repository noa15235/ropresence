//! Build a Discord [`ActivityPayload`] from the current config + runtime state.
//!
//! This is the single place that maps Roblox state -> Discord card. It is pure
//! (no I/O), which keeps the worker loop simple and makes diffing reliable.

use crate::config::AppConfig;
use crate::discord::ActivityPayload;
use crate::presence::variables::{resolve, VarContext};
use crate::state::{unix_now, RuntimeState};

fn nonempty(s: &str) -> Option<String> {
    let t = s.trim();
    if t.is_empty() {
        None
    } else {
        Some(t.to_string())
    }
}

/// The username to display, honouring multi-account selection and privacy mode.
pub fn active_username(cfg: &AppConfig) -> String {
    if cfg.privacy_mode {
        return String::new();
    }
    cfg.roblox
        .accounts
        .get(cfg.roblox.active_account)
        .cloned()
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| cfg.roblox.username.clone())
}

fn make_ctx(cfg: &AppConfig, rt: &RuntimeState) -> VarContext {
    VarContext {
        game: rt.game_name.clone().unwrap_or_default(),
        creator: rt.creator_name.clone().unwrap_or_default(),
        username: active_username(cfg),
        user_id: rt.user_id.map(|x| x.to_string()).unwrap_or_default(),
        place_id: rt.place_id.map(|x| x.to_string()).unwrap_or_default(),
        universe_id: rt.universe_id.map(|x| x.to_string()).unwrap_or_default(),
        players: rt.player_count.map(|x| x.to_string()).unwrap_or_default(),
        job_id: rt.job_id.clone().unwrap_or_default(),
        elapsed_secs: rt.session_start.map(|s| (unix_now() - s).max(0)),
    }
}

fn large_image(cfg: &AppConfig, rt: &RuntimeState) -> Option<String> {
    match cfg.presence.large_image_mode.as_str() {
        // A custom asset key or URL set by the user.
        "asset" | "url" => nonempty(&cfg.presence.large_image_key),
        // "auto": use the live game icon URL (Discord RPC accepts external URLs
        // in large_image with a valid app id). When no game is detected this is
        // None, and Discord falls back to the application icon (Roblox logo).
        _ => rt.game_icon_url.clone(),
    }
}

fn small_image(cfg: &AppConfig, rt: &RuntimeState) -> Option<String> {
    match cfg.presence.small_image_mode.as_str() {
        "asset" | "url" => nonempty(&cfg.presence.small_image_key),
        "avatar" if !cfg.privacy_mode => rt.avatar_url.clone(),
        _ => None,
    }
}

/// Build the activity, or `None` to clear the presence.
pub fn build(cfg: &AppConfig, rt: &RuntimeState) -> Option<ActivityPayload> {
    if !cfg.master_enabled || cfg.discord_client_id.is_empty() {
        return None;
    }

    let studio = rt.is_studio && cfg.roblox.detect_studio;
    let running = rt.roblox_running || studio;

    if !running {
        return match cfg.roblox.fallback_when_closed.as_str() {
            "static" => Some(build_static(cfg)),
            _ => None,
        };
    }

    let ctx = make_ctx(cfg, rt);
    let f = &cfg.features;
    let pr = &cfg.presence;
    let mut p = ActivityPayload::default();

    // Choose text templates by mode: in-game / studio / menu.
    let (details_tpl, state_tpl) = if rt.in_game {
        (pr.details.clone(), pr.state.clone())
    } else if studio {
        ("Roblox Studio".to_string(), "En train de créer".to_string())
    } else {
        ("Roblox".to_string(), "Dans le menu".to_string())
    };

    if f.show_details {
        p.details = nonempty(&resolve(&details_tpl, &ctx));
    }
    if f.show_state {
        p.state = nonempty(&resolve(&state_tpl, &ctx));
    }

    if f.show_large_image {
        p.large_image = large_image(cfg, rt);
        // Only attach hover text when there's an actual asset image; otherwise
        // keep the assets minimal so Discord shows the application icon fallback.
        if p.large_image.is_some() {
            p.large_text = nonempty(&resolve(&pr.large_image_text, &ctx));
        }
    }
    if f.show_small_image {
        p.small_image = small_image(cfg, rt);
        p.small_text = nonempty(&resolve(&pr.small_image_text, &ctx));
    }

    if f.show_timer {
        p.start_timestamp = rt.session_start;
    }

    if f.show_buttons {
        let mut buttons: Vec<(String, String)> = Vec::new();
        for b in &pr.buttons {
            let label = resolve(&b.label, &ctx);
            let url = resolve(&b.url, &ctx);
            // Skip buttons whose variables didn't resolve (leftover `{...}`) or
            // that left an empty path segment, e.g. `users//profile` when no
            // username/userId is set — those would be dead links on Discord.
            let path_ok = url
                .splitn(2, "://")
                .nth(1)
                .map_or(false, |rest| !rest.contains("//"));
            if !label.is_empty() && url.starts_with("http") && !url.contains('{') && path_ok {
                buttons.push((label, url));
            }
        }
        // Auto button: the experience page (#27).
        if f.auto_buttons && rt.in_game {
            if let Some(pid) = rt.place_id {
                if buttons.len() < 2 {
                    buttons.push((
                        "Voir l'expérience".to_string(),
                        format!("https://www.roblox.com/games/{pid}"),
                    ));
                }
            }
        }
        buttons.truncate(2);
        p.buttons = buttons;
    }

    if f.show_party {
        if let (Some(cur), Some(max)) = (rt.player_count, rt.max_players) {
            if max > 0 {
                p.party = Some((cur.min(u32::MAX as u64) as u32, max.min(u32::MAX as u64) as u32));
            }
        }
    }

    Some(p)
}

/// Static presence used when Roblox is closed and fallback = "static" (#36).
fn build_static(cfg: &AppConfig) -> ActivityPayload {
    let ctx = VarContext {
        username: active_username(cfg),
        ..Default::default()
    };
    let mut p = ActivityPayload::default();
    let f = &cfg.features;
    let pr = &cfg.presence;

    if f.show_details {
        p.details = nonempty(&resolve(&cfg.roblox.static_details, &ctx));
    }
    if f.show_state {
        p.state = nonempty(&resolve(&cfg.roblox.static_state, &ctx));
    }
    if f.show_large_image {
        // No live game icon when closed: only an explicit asset/url key applies.
        if matches!(pr.large_image_mode.as_str(), "asset" | "url") {
            p.large_image = nonempty(&pr.large_image_key);
            p.large_text = nonempty(&resolve(&pr.large_image_text, &ctx));
        }
    }
    p
}
