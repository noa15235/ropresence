use discord_rich_presence::activity::{Activity, Assets, Button, Party, Timestamps};
use discord_rich_presence::{DiscordIpc, DiscordIpcClient};

fn usable_asset(value: &str) -> bool {
    !value.trim().is_empty()
}

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

    pub fn is_connected_to(&self, client_id: &str) -> bool {
        self.connected && self.client_id == client_id
    }

    pub fn connect(&mut self, client_id: &str) -> Result<(), String> {
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

    pub fn mark_disconnected(&mut self) {
        self.client = None;
        self.connected = false;
    }

    pub fn disconnect(&mut self) {
        if let Some(mut client) = self.client.take() {
            let _ = client.close();
        }
        self.connected = false;
    }

    pub fn clear(&mut self) -> Result<(), String> {
        if let Some(client) = self.client.as_mut() {
            client
                .clear_activity()
                .map_err(|e| format!("Échec de la réinitialisation : {e}"))?;
        }
        Ok(())
    }

    pub fn set_activity(&mut self, p: &ActivityPayload) -> Result<(), String> {
        let client = self.client.as_mut().ok_or("Discord non connecté")?;

        let mut activity = Activity::new();

        if let Some(d) = p.details.as_deref().filter(|s| !s.is_empty()) {
            activity = activity.details(d);
        }
        if let Some(s) = p.state.as_deref().filter(|s| !s.is_empty()) {
            activity = activity.state(s);
        }

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
