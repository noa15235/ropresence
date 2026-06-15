//! Application configuration model.
//!
//! This mirrors the `AppConfig` TypeScript interface used by the front-end.
//! Everything is serialised in `camelCase` so the same JSON travels unchanged
//! between Rust and the UI, and is persisted by `tauri-plugin-store`.

pub mod store;

use serde::{Deserialize, Serialize};

/// Discord's built-in Roblox application id. Used as a zero-setup default so
/// Rich Presence works out of the box (the card shows "ROBLOX"). Users can
/// override it with their own application id for a custom name and images.
pub const DEFAULT_CLIENT_ID: &str = "363445589247131668";

/// A single Discord activity button (max 2 enforced when building the payload).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PresenceButton {
    pub label: String,
    pub url: String,
}

/// Per-feature on/off switches (#17). Defaults are all-on (party is opt-in).
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

/// Visual presentation of the Discord card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct PresenceConfig {
    /// First line on the Discord card. Supports variables.
    pub details: String,
    /// Second line on the Discord card. Supports variables.
    pub state: String,
    /// "auto" (live game icon URL), "asset" (Discord asset key), or "url".
    pub large_image_mode: String,
    pub large_image_key: String,
    pub large_image_text: String,
    /// "none", "asset", "url", or "avatar" (Roblox headshot).
    pub small_image_mode: String,
    pub small_image_key: String,
    pub small_image_text: String,
    /// Custom buttons (max 2, Discord limit). Auto buttons are added separately.
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

/// Roblox detection settings.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct RobloxConfig {
    pub username: String,
    pub accounts: Vec<String>,
    pub active_account: usize,
    pub detect_studio: bool,
    /// "clear" or "static".
    pub fallback_when_closed: String,
    pub static_details: String,
    pub static_state: String,
    /// Seconds between log/process polls (clamped 2..=15 at runtime).
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
    /// "dark" or "light".
    pub theme: String,
    /// Accent colour, hex string.
    pub accent: String,
    /// "fr" or "en".
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
    /// Global hotkey to toggle the master switch (e.g. "CommandOrControl+Shift+R").
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

/// A saved, named presence profile (#35).
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

/// Root configuration object, persisted as a single JSON blob.
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
            setup_complete: false,
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
