use super::AppConfig;
use serde::{Deserialize, Serialize};
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;

const STORE_FILE: &str = "config.json";
const CONFIG_KEY: &str = "config";
const STATS_KEY: &str = "stats";

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DailyStats {
    pub date: String,
    pub seconds: u64,
}

pub fn load_stats(app: &AppHandle) -> DailyStats {
    match app.store(STORE_FILE) {
        Ok(store) => store
            .get(STATS_KEY)
            .and_then(|v| serde_json::from_value(v).ok())
            .unwrap_or_default(),
        Err(_) => DailyStats::default(),
    }
}

pub fn save_stats(app: &AppHandle, stats: &DailyStats) {
    if let Ok(store) = app.store(STORE_FILE) {
        if let Ok(value) = serde_json::to_value(stats) {
            store.set(STATS_KEY, value);
            let _ = store.save();
        }
    }
}

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
