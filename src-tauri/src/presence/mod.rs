//! Presence orchestration: the background worker that maps Roblox state to a
//! Discord activity, manages the connection, and emits live updates to the UI.

pub mod builder;
pub mod variables;

use crate::discord::{ActivityPayload, Backoff, DiscordManager};
use crate::roblox::{api, log_parser, process_watch};
use crate::state::{unix_now, AppState, LogLevel, RuntimeState};
use std::sync::Arc;
use std::time::Duration;
use tauri::{AppHandle, Emitter};
use tauri_plugin_notification::NotificationExt;

fn nonempty_opt(s: String) -> Option<String> {
    if s.trim().is_empty() {
        None
    } else {
        Some(s)
    }
}

fn notify(app: &AppHandle, title: &str, body: &str) {
    let _ = app.notification().builder().title(title).body(body).show();
}

/// The main background loop. Runs on a dedicated std thread for the app's life.
pub fn run_worker(app: AppHandle, state: Arc<AppState>) {
    let mut discord = DiscordManager::new();
    let mut backoff = Backoff::new();
    let mut last_payload: Option<ActivityPayload> = None;
    let mut session_start: Option<i64> = None;
    let mut game_cache: Option<(u64, api::GameInfo)> = None;
    let mut avatar_cache: Option<(String, Option<String>)> = None;
    let mut userid_cache: Option<(String, Option<u64>)> = None;
    let mut current_id = String::new();
    let mut prev_in_game = false;
    let mut prev_connected = false;

    state.log(LogLevel::Info, "app", "Worker démarré");

    loop {
        let cfg = { state.config.lock().unwrap().clone() };

        // Force an immediate (re)connection when the UI asks (one-click connect).
        if state
            .force_reconnect
            .swap(false, std::sync::atomic::Ordering::SeqCst)
        {
            backoff.reset();
            discord.disconnect();
            last_payload = None;
        }

        // --- Detect Roblox ------------------------------------------------
        let procs = process_watch::detect();
        let running = procs.any();

        // Session timer anchor: set when Roblox starts, cleared when it stops.
        if running && session_start.is_none() {
            session_start = Some(unix_now());
        } else if !running {
            session_start = None;
            game_cache = None;
        }

        let mut rt = RuntimeState {
            roblox_running: procs.player,
            is_studio: procs.studio,
            session_start,
            ..Default::default()
        };

        // --- Parse logs + resolve game info -------------------------------
        if procs.player {
            if let Some(parsed) = log_parser::parse_latest() {
                rt.in_game = parsed.in_game && parsed.place_id.is_some();
                rt.place_id = parsed.place_id;
                rt.universe_id = parsed.universe_id;
                rt.job_id = parsed.job_id;

                if rt.in_game {
                    if let Some(pid) = rt.place_id {
                        let cached = game_cache
                            .as_ref()
                            .filter(|(p, _)| *p == pid)
                            .map(|(_, g)| g.clone());
                        let info = match cached {
                            Some(g) => Some(g),
                            None => {
                                let fetched = api::fetch_game_info(pid, rt.universe_id);
                                match &fetched {
                                    Some(g) => {
                                        game_cache = Some((pid, g.clone()));
                                        state.log(
                                            LogLevel::Info,
                                            "roblox",
                                            format!("Expérience : {} (place {})", g.name, pid),
                                        );
                                    }
                                    None => state.log(
                                        LogLevel::Warn,
                                        "roblox",
                                        format!("API Roblox indisponible pour la place {pid}"),
                                    ),
                                }
                                fetched
                            }
                        };
                        if let Some(g) = info {
                            rt.universe_id = Some(g.universe_id);
                            rt.game_name = nonempty_opt(g.name);
                            rt.creator_name = nonempty_opt(g.creator_name);
                            rt.game_icon_url = g.icon_url;
                            rt.player_count = g.playing;
                            rt.max_players = g.max_players;
                        }
                    }
                }
            }
        }

        // --- Roblox user id (for {userId}, profile button) + avatar -------
        if !cfg.privacy_mode {
            let username = builder::active_username(&cfg);
            if !username.is_empty() {
                let uid = match userid_cache.as_ref().filter(|(u, _)| *u == username) {
                    Some((_, id)) => *id,
                    None => {
                        let id = api::resolve_user_id(&username);
                        userid_cache = Some((username.clone(), id));
                        id
                    }
                };
                rt.user_id = uid;

                if cfg.presence.small_image_mode == "avatar" {
                    if let Some(id) = uid {
                        rt.avatar_url = match avatar_cache.as_ref().filter(|(u, _)| *u == username) {
                            Some((_, a)) => a.clone(),
                            None => {
                                let url = api::fetch_avatar(id);
                                avatar_cache = Some((username.clone(), url.clone()));
                                url
                            }
                        };
                    }
                }
            }
        }

        // --- Discord connection management --------------------------------
        let want_discord = cfg.master_enabled && !cfg.discord_client_id.is_empty();

        if cfg.discord_client_id != current_id {
            current_id = cfg.discord_client_id.clone();
            backoff.reset();
            last_payload = None;
            discord.disconnect();
        }

        if want_discord {
            if !discord.is_connected_to(&cfg.discord_client_id) && backoff.ready() {
                match discord.connect(&cfg.discord_client_id) {
                    Ok(()) => {
                        backoff.record(true);
                        state.log(LogLevel::Info, "discord", "Connecté à Discord");
                        last_payload = None;
                        // Let Discord finish the handshake before the first push,
                        // otherwise the first SET_ACTIVITY can race and close the pipe.
                        std::thread::sleep(std::time::Duration::from_millis(400));
                    }
                    Err(e) => {
                        backoff.record(false);
                        rt.last_error = Some(e.clone());
                        state.log(LogLevel::Warn, "discord", format!("Connexion échouée : {e}"));
                    }
                }
            }
        } else if discord.connected {
            let _ = discord.clear();
            discord.disconnect();
            last_payload = None;
        }

        // --- Push activity (only on change) -------------------------------
        if discord.connected {
            let payload = builder::build(&cfg, &rt);
            if payload != last_payload {
                match &payload {
                    Some(p) => match discord.set_activity(p) {
                        Ok(()) => {
                            rt.last_error = None;
                        }
                        Err(e) => {
                            let friendly = if e.contains("pipe is being closed")
                                || e.contains("os error 232")
                            {
                                "Client ID invalide ou inexistant : Discord a refusé l'activité. Vérifie ton Application ID dans les Réglages.".to_string()
                            } else {
                                e.clone()
                            };
                            rt.last_error = Some(friendly.clone());
                            state.log(LogLevel::Warn, "discord", friendly);
                            // Drop the connection; the backoff governs the next
                            // attempt so we never hammer Discord's rate limit.
                            discord.mark_disconnected();
                            backoff.record(false);
                        }
                    },
                    None => {
                        let _ = discord.clear();
                    }
                }
                if discord.connected {
                    last_payload = payload;
                }
            }
        }

        rt.discord_connected = discord.connected;

        // --- Notifications on transitions ---------------------------------
        if cfg.system.notifications {
            if rt.in_game && !prev_in_game {
                let name = rt.game_name.clone().unwrap_or_else(|| "Roblox".to_string());
                notify(&app, "Jeu détecté", &format!("Vous jouez à {name}"));
            }
            if prev_connected && !discord.connected && want_discord {
                notify(&app, "Discord déconnecté", "Tentative de reconnexion…");
            }
        }
        prev_in_game = rt.in_game;
        prev_connected = discord.connected;

        // --- Publish runtime state to the UI ------------------------------
        {
            *state.runtime.lock().unwrap() = rt.clone();
        }
        let _ = app.emit("runtime-update", &rt);

        // --- Wait (wake early on config change, stop on shutdown) ----------
        let poll = cfg.roblox.poll_interval_secs.clamp(2, 15);
        let mut sig = state.signal.lock().unwrap();
        if sig.stop {
            break;
        }
        if !sig.wake {
            let (guard, _) = state
                .condvar
                .wait_timeout(sig, Duration::from_secs(poll))
                .unwrap();
            sig = guard;
        }
        sig.wake = false;
        let stop = sig.stop;
        drop(sig);
        if stop {
            break;
        }
    }

    // Clean shutdown.
    let _ = discord.clear();
    discord.disconnect();
    state.log(LogLevel::Info, "app", "Worker arrêté");
}
