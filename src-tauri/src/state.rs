use crate::config::AppConfig;
use serde::Serialize;
use std::sync::atomic::AtomicBool;
use std::sync::{Condvar, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn unix_now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

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
    pub user_id: Option<u64>,
    pub player_count: Option<u64>,
    pub max_players: Option<u64>,
    pub session_start: Option<i64>,
    pub game_start: Option<i64>,
    pub daily_seconds: Option<u64>,
    pub last_error: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Info,
    Warn,
    Error,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LogEntry {
    pub ts: i64,
    pub level: LogLevel,
    pub scope: String,
    pub message: String,
}

#[derive(Default)]
pub struct WorkerSignal {
    pub wake: bool,
    pub stop: bool,
}

const MAX_LOGS: usize = 300;

pub struct AppState {
    pub config: Mutex<AppConfig>,
    pub runtime: Mutex<RuntimeState>,
    pub logs: Mutex<Vec<LogEntry>>,
    pub signal: Mutex<WorkerSignal>,
    pub condvar: Condvar,
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

    pub fn notify_worker(&self) {
        if let Ok(mut sig) = self.signal.lock() {
            sig.wake = true;
            self.condvar.notify_all();
        }
    }

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
