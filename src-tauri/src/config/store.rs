//! Persistence of [`AppConfig`] via `tauri-plugin-store`.
//!
//! The whole config is stored under a single `"config"` key in `config.json`
//! inside the app data directory. Loads are tolerant: a missing or corrupt
//! file falls back to defaults instead of panicking.

use super::AppConfig;
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;

const STORE_FILE: &str = "config.json";
const CONFIG_KEY: &str = "config";

/// Load the config from disk, falling back to defaults on any error.
pub fn load_config(app: &AppHandle) -> AppConfig {
    let store = match app.store(STORE_FILE) {
        Ok(s) => s,
        Err(_) => return AppConfig::default(),
    };

    match store.get(CONFIG_KEY) {
        Some(value) => serde_json::from_value(value).unwrap_or_default(),
        None => AppConfig::default(),
    }
}

/// Persist the config to disk. Returns a French error message on failure.
pub fn save_config(app: &AppHandle, config: &AppConfig) -> Result<(), String> {
    let store = app
        .store(STORE_FILE)
        .map_err(|e| format!("Impossible d'ouvrir le stockage : {e}"))?;

    let value = serde_json::to_value(config)
        .map_err(|e| format!("Impossible de sérialiser la configuration : {e}"))?;

    store.set(CONFIG_KEY, value);
    store
        .save()
        .map_err(|e| format!("Impossible d'enregistrer la configuration : {e}"))?;
    Ok(())
}
