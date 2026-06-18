pub mod store;

use serde::{Deserialize, Serialize};

pub const DEFAULT_CLIENT_ID: &str = "363445589247131668";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PresenceButton {
    pub label: String,
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct FeatureFlags {
    pub show_details: bool,
    pub show_state: bool,
    pub show_timer: bool,
    pub show_large_image: bool,
    pub show_small_image: bool,
    pub show_buttons: bool,
    pub show_party: bool,
    pub auto_buttons: bool,
}

impl Default for FeatureFlags {
    fn default() -> Self {
        Self {
            show_details: true,
            show_state: true,
            show_timer: true,
            show_large_image: true,
            show_small_image: true,
            show_buttons: true,
            show_party: false,
            auto_buttons: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct PresenceConfig {
    pub details: String,
    pub state: String,
    pub large_image_mode: String,
    pub large_image_key: String,
    pub large_image_text: String,
    pub small_image_mode: String,
    pub small_image_key: String,
    pub small_image_text: String,
    pub buttons: Vec<PresenceButton>,
}

impl Default for PresenceConfig {
    fn default() -> Self {
        Self {
            details: "{game}".to_string(),
            state: "par {creator}".to_string(),
            large_image_mode: "auto".to_string(),
            large_image_key: "roblox".to_string(),
            large_image_text: "{game}".to_string(),
            small_image_mode: "none".to_string(),
            small_image_key: "".to_string(),
            small_image_text: "{username}".to_string(),
            buttons: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct RobloxConfig {
    pub username: String,
    pub accounts: Vec<String>,
    pub active_account: usize,
    pub detect_studio: bool,
    pub fallback_when_closed: String,
    pub static_details: String,
    pub static_state: String,
    pub poll_interval_secs: u64,
}

impl Default for RobloxConfig {
    fn default() -> Self {
        Self {
            username: String::new(),
            accounts: Vec::new(),
            active_account: 0,
            detect_studio: true,
            fallback_when_closed: "clear".to_string(),
            static_details: "Sur le bureau".to_string(),
            static_state: "En attente de Roblox…".to_string(),
            poll_interval_secs: 4,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Appearance {
    pub theme: String,
    pub accent: String,
    pub language: String,
}

impl Default for Appearance {
    fn default() -> Self {
        Self {
            theme: "dark".to_string(),
            accent: "#2E9BFF".to_string(),
            language: "fr".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct SystemConfig {
    pub autostart: bool,
    pub start_minimized: bool,
    pub close_to_tray: bool,
    pub notifications: bool,
    pub hotkey_toggle: String,
}

impl Default for SystemConfig {
    fn default() -> Self {
        Self {
            autostart: false,
            start_minimized: false,
            close_to_tray: true,
            notifications: true,
            hotkey_toggle: String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct PresenceProfile {
    pub id: String,
    pub name: String,
    pub presence: PresenceConfig,
    pub features: FeatureFlags,
}

impl Default for PresenceProfile {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            presence: PresenceConfig::default(),
            features: FeatureFlags::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct AppConfig {
    pub discord_client_id: String,
    pub master_enabled: bool,
    pub privacy_mode: bool,
    pub setup_complete: bool,
    pub presence: PresenceConfig,
    pub roblox: RobloxConfig,
    pub features: FeatureFlags,
    pub appearance: Appearance,
    pub system: SystemConfig,
    pub profiles: Vec<PresenceProfile>,
    pub active_profile: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            discord_client_id: DEFAULT_CLIENT_ID.to_string(),
            master_enabled: true,
            privacy_mode: false,
            // true : dès l'installation, l'app utilise l'Application ID Discord
            // intégré (DEFAULT_CLIENT_ID) et se connecte automatiquement, sans
            // forcer l'assistant de création d'app. Créer sa propre app reste
            // possible et optionnel via Réglages → Discord (pour le branding).
            setup_complete: true,
            presence: PresenceConfig::default(),
            roblox: RobloxConfig::default(),
            features: FeatureFlags::default(),
            appearance: Appearance::default(),
            system: SystemConfig::default(),
            profiles: Vec::new(),
            active_profile: String::new(),
        }
    }
}
