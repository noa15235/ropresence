//! Tauri commands exposed to the front-end. These are intentionally thin and
//! delegate to the config/state/worker layers.

use crate::config::{store, AppConfig};
use crate::presence::variables::SUPPORTED_VARIABLES;
use crate::state::{AppState, LogEntry, RuntimeState};
use std::sync::atomic::Ordering;
use std::sync::Arc;
use tauri::{AppHandle, Manager, State};

type Shared<'a> = State<'a, Arc<AppState>>;

#[tauri::command]
pub fn get_config(state: Shared) -> AppConfig {
    state.config.lock().unwrap().clone()
}

#[tauri::command]
pub fn set_config(app: AppHandle, state: Shared, config: AppConfig) -> Result<(), String> {
    {
        *state.config.lock().unwrap() = config.clone();
    }
    store::save_config(&app, &config)?;
    state.notify_worker();
    Ok(())
}

#[tauri::command]
pub fn get_runtime(state: Shared) -> RuntimeState {
    state.runtime.lock().unwrap().clone()
}

#[tauri::command]
pub fn get_variables() -> Vec<String> {
    SUPPORTED_VARIABLES.iter().map(|s| s.to_string()).collect()
}

/// Toggle the global master switch and persist. Returns the new value.
#[tauri::command]
pub fn toggle_master(app: AppHandle, state: Shared) -> Result<bool, String> {
    let new_value = {
        let mut cfg = state.config.lock().unwrap();
        cfg.master_enabled = !cfg.master_enabled;
        cfg.master_enabled
    };
    let snapshot = state.config.lock().unwrap().clone();
    store::save_config(&app, &snapshot)?;
    state.notify_worker();
    Ok(new_value)
}

/// Force an immediate Discord (re)connection: enables the master switch,
/// resets the backoff and wakes the worker so it connects right now (#1).
#[tauri::command]
pub fn reconnect_discord(app: AppHandle, state: Shared) -> Result<(), String> {
    {
        let mut cfg = state.config.lock().unwrap();
        cfg.master_enabled = true;
    }
    let snapshot = state.config.lock().unwrap().clone();
    store::save_config(&app, &snapshot)?;
    state.force_reconnect.store(true, Ordering::SeqCst);
    state.notify_worker();
    Ok(())
}

/// Validate a Discord application/client id: format check, then a real
/// existence check against Discord. Returns the application name on success.
#[tauri::command]
pub fn validate_client_id(client_id: String) -> Result<String, String> {
    let id = client_id.trim();
    if id.is_empty() {
        return Err("Veuillez saisir un Client ID.".to_string());
    }
    if !id.chars().all(|c| c.is_ascii_digit()) {
        return Err("Le Client ID ne doit contenir que des chiffres.".to_string());
    }
    if !(17..=20).contains(&id.len()) {
        return Err("Le Client ID doit comporter entre 17 et 20 chiffres.".to_string());
    }
    // Format is valid — now do a real RPC probe to confirm the app exists.
    crate::discord::test_connection(id)
}

/// Open a URL in the user's default browser (http/https only).
#[tauri::command]
pub fn open_url(url: String) -> Result<(), String> {
    if !(url.starts_with("http://") || url.starts_with("https://")) {
        return Err("URL non valide.".to_string());
    }
    #[cfg(windows)]
    {
        std::process::Command::new("cmd")
            .args(["/C", "start", "", &url])
            .spawn()
            .map_err(|e| format!("Impossible d'ouvrir le lien : {e}"))?;
    }
    #[cfg(not(windows))]
    {
        let _ = url;
    }
    Ok(())
}

#[tauri::command]
pub fn get_logs(state: Shared) -> Vec<LogEntry> {
    state.logs.lock().unwrap().clone()
}

#[tauri::command]
pub fn clear_logs(state: Shared) {
    state.logs.lock().unwrap().clear();
}

/// Export the current config to a JSON file path (#43).
#[tauri::command]
pub fn export_config(state: Shared, path: String) -> Result<(), String> {
    let cfg = state.config.lock().unwrap().clone();
    let json = serde_json::to_string_pretty(&cfg)
        .map_err(|e| format!("Sérialisation impossible : {e}"))?;
    std::fs::write(&path, json).map_err(|e| format!("Écriture impossible : {e}"))?;
    Ok(())
}

/// Import a config from a JSON file path and apply it (#43).
#[tauri::command]
pub fn import_config(app: AppHandle, state: Shared, path: String) -> Result<AppConfig, String> {
    let text = std::fs::read_to_string(&path).map_err(|e| format!("Lecture impossible : {e}"))?;
    let cfg: AppConfig =
        serde_json::from_str(&text).map_err(|e| format!("Fichier de config invalide : {e}"))?;
    {
        *state.config.lock().unwrap() = cfg.clone();
    }
    store::save_config(&app, &cfg)?;
    state.notify_worker();
    Ok(cfg)
}

#[tauri::command]
pub fn show_main_window(app: AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.unminimize();
        let _ = window.show();
        let _ = window.set_focus();
    }
}

#[tauri::command]
pub fn quit_app(app: AppHandle) {
    // Exiting closes the IPC pipe, which makes Discord auto-clear the presence.
    app.exit(0);
}
