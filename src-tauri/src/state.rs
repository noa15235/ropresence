//! Shared application state and the worker wake/stop signalling primitives.

use crate::config::AppConfig;
use serde::Serialize;
use std::sync::atomic::AtomicBool;
use std::sync::{Condvar, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

/// Current unix time in seconds.
pub fn unix_now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

/// Live runtime state, emitted to the front-end as `runtime-update` events.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeState {
    pub discord_connected: bool,
    pub roblox_running: bool,
    pub in_game: bool,
    pub is_studio: bool,
    pub place_id: Option<u64>,
    pub universe_id: Option<u64>,
    pub job_id: Option<String>,
    pub game_name: Option<String>,
    pub creator_name: Option<String>,
    pub game_icon_url: Option<String>,
    pub avatar_url: Option<String>,
    /// Resolved Roblox user id for the active account (for `{userId}` / profile button).
    pub user_id: Option<u64>,
    pub player_count: Option<u64>,
    pub max_players: Option<u64>,
    /// Unix seconds when the current game session started (timer anchor).
    pub session_start: Option<i64>,
    /// Last non-fatal error message (shown in the debug panel / status).
    pub last_error: Option<String>,
}

/// Severity for a debug log entry.
#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Info,
    Warn,
    Error,
}

/// A single debug log line (#47).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LogEntry {
    pub ts: i64,
    pub level: LogLevel,
    pub scope: String,
    pub message: String,
}

/// Worker wake/stop signal protected by a condvar.
#[derive(Default)]
pub struct WorkerSignal {
    pub wake: bool,
    pub stop: bool,
}

const MAX_LOGS: usize = 300;

/// Process-wide shared state. Stored in Tauri's managed state as `Arc<AppState>`.
pub struct AppState {
    pub config: Mutex<AppConfig>,
    pub runtime: Mutex<RuntimeState>,
    pub logs: Mutex<Vec<LogEntry>>,
    pub signal: Mutex<WorkerSignal>,
    pub condvar: Condvar,
    /// Set by the UI to force an immediate Discord (re)connection (#1 easy connect).
    pub force_reconnect: AtomicBool,
}

impl AppState {
    pub fn new(config: AppConfig) -> Self {
        Self {
            config: Mutex::new(config),
            runtime: Mutex::new(RuntimeState::default()),
            logs: Mutex::new(Vec::new()),
            signal: Mutex::new(WorkerSignal::default()),
            condvar: Condvar::new(),
            force_reconnect: AtomicBool::new(false),
        }
    }

    /// Wake the worker immediately (e.g. after a live config edit).
    pub fn notify_worker(&self) {
        if let Ok(mut sig) = self.signal.lock() {
            sig.wake = true;
            self.condvar.notify_all();
        }
    }

    /// Ask the worker to stop and wake it.
    pub fn stop_worker(&self) {
        if let Ok(mut sig) = self.signal.lock() {
            sig.stop = true;
            self.condvar.notify_all();
        }
    }

    /// Push a debug log entry (ring buffer).
    pub fn log(&self, level: LogLevel, scope: &str, message: impl Into<String>) {
        if let Ok(mut logs) = self.logs.lock() {
            logs.push(LogEntry {
                ts: unix_now(),
                level,
                scope: scope.to_string(),
                message: message.into(),
            });
            let len = logs.len();
            if len > MAX_LOGS {
                logs.drain(0..len - MAX_LOGS);
            }
        }
    }
}
