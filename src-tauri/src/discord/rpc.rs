//! Thin wrapper around `discord-rich-presence` that owns the IPC client and
//! exposes a small, diff-friendly [`ActivityPayload`].

use discord_rich_presence::activity::{Activity, Assets, Button, Party, Timestamps};
use discord_rich_presence::{DiscordIpc, DiscordIpcClient};

/// A Discord asset image value is either an uploaded asset *key* or — with a
/// valid application id — an external image **URL** (Discord's RPC supports
/// external URLs in `large_image`/`small_image`). Both are accepted; we only
/// reject empties.
fn usable_asset(value: &str) -> bool {
    !value.trim().is_empty()
}

/// An owned, comparable description of a Discord activity. The worker builds
/// one of these each tick and only pushes to Discord when it changes (#24).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ActivityPayload {
    pub details: Option<String>,
    pub state: Option<String>,
    pub large_image: Option<String>,
    pub large_text: Option<String>,
    pub small_image: Option<String>,
    pub small_text: Option<String>,
    pub start_timestamp: Option<i64>,
    pub buttons: Vec<(String, String)>,
    pub party: Option<(u32, u32)>,
}

/// Owns the Discord IPC client and tracks connection state.
pub struct DiscordManager {
    client: Option<DiscordIpcClient>,
    client_id: String,
    pub connected: bool,
}

impl DiscordManager {
    pub fn new() -> Self {
        Self {
            client: None,
            client_id: String::new(),
            connected: false,
        }
    }

    /// True when connected and bound to the given client id.
    pub fn is_connected_to(&self, client_id: &str) -> bool {
        self.connected && self.client_id == client_id
    }

    /// (Re)connect to Discord with the given application/client id.
    pub fn connect(&mut self, client_id: &str) -> Result<(), String> {
        // Drop any previous client (e.g. when the client id changed).
        self.disconnect();

        let mut client =
            DiscordIpcClient::new(client_id).map_err(|e| format!("Client ID invalide : {e}"))?;
        client
            .connect()
            .map_err(|e| format!("Connexion à Discord impossible : {e}"))?;

        self.client = Some(client);
        self.client_id = client_id.to_string();
        self.connected = true;
        Ok(())
    }

    /// Mark the connection as lost (next tick will attempt a reconnect).
    pub fn mark_disconnected(&mut self) {
        self.client = None;
        self.connected = false;
    }

    /// Close and forget the client.
    pub fn disconnect(&mut self) {
        if let Some(mut client) = self.client.take() {
            let _ = client.close();
        }
        self.connected = false;
    }

    /// Clear the activity (presence removed from the profile).
    pub fn clear(&mut self) -> Result<(), String> {
        if let Some(client) = self.client.as_mut() {
            client
                .clear_activity()
                .map_err(|e| format!("Échec de la réinitialisation : {e}"))?;
        }
        Ok(())
    }

    /// Push an activity to Discord. All borrowed strings live in `p`, which
    /// outlives the borrow, so the lifetime-bound builder is safe here.
    pub fn set_activity(&mut self, p: &ActivityPayload) -> Result<(), String> {
        let client = self.client.as_mut().ok_or("Discord non connecté")?;

        let mut activity = Activity::new();

        if let Some(d) = p.details.as_deref().filter(|s| !s.is_empty()) {
            activity = activity.details(d);
        }
        if let Some(s) = p.state.as_deref().filter(|s| !s.is_empty()) {
            activity = activity.state(s);
        }

        // Assets (large/small image + hover text). Image values that are URLs
        // are dropped (Discord IPC only accepts asset keys — see usable_asset).
        let mut assets = Assets::new();
        let mut has_assets = false;
        if let Some(v) = p.large_image.as_deref().filter(|s| usable_asset(s)) {
            assets = assets.large_image(v);
            has_assets = true;
        }
        if let Some(v) = p.large_text.as_deref().filter(|s| !s.is_empty()) {
            assets = assets.large_text(v);
            has_assets = true;
        }
        if let Some(v) = p.small_image.as_deref().filter(|s| usable_asset(s)) {
            assets = assets.small_image(v);
            has_assets = true;
        }
        if let Some(v) = p.small_text.as_deref().filter(|s| !s.is_empty()) {
            assets = assets.small_text(v);
            has_assets = true;
        }
        if has_assets {
            activity = activity.assets(assets);
        }

        if let Some(ts) = p.start_timestamp {
            activity = activity.timestamps(Timestamps::new().start(ts));
        }

        // Buttons (max 2 enforced upstream).
        let buttons: Vec<Button> = p
            .buttons
            .iter()
            .filter(|(l, u)| !l.is_empty() && !u.is_empty())
            .map(|(l, u)| Button::new(l, u))
            .collect();
        if !buttons.is_empty() {
            activity = activity.buttons(buttons);
        }

        if let Some((cur, max)) = p.party {
            if max > 0 {
                activity = activity.party(Party::new().size([cur as i32, max as i32]));
            }
        }

        client
            .set_activity(activity)
            .map_err(|e| format!("Échec d'envoi de l'activité : {e}"))?;
        Ok(())
    }
}

impl Default for DiscordManager {
    fn default() -> Self {
        Self::new()
    }
}
