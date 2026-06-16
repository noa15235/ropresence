pub mod reconnect;
pub mod rpc;

pub use reconnect::Backoff;
pub use rpc::{ActivityPayload, DiscordManager};

use discord_rich_presence::activity::Activity;
use discord_rich_presence::{DiscordIpc, DiscordIpcClient};
use std::time::Duration;

pub fn test_connection(client_id: &str) -> Result<String, String> {
    let mut client = DiscordIpcClient::new(client_id)
        .map_err(|e| format!("Client ID invalide : {e}"))?;
    client.connect().map_err(|_| {
        "Discord est introuvable. Ouvre l'application Discord puis réessaie.".to_string()
    })?;
    std::thread::sleep(Duration::from_millis(600));
    let probe = Activity::new().state("RoPresence");
    let result = client.set_activity(probe).map_err(|e| e.to_string());
    let _ = client.clear_activity();
    let _ = client.close();

    match result {
        Ok(()) => Ok(application_name(client_id).unwrap_or_default()),
        Err(e) if e.contains("pipe") || e.contains("232") => Err(
            "Ce Client ID n'existe pas sur Discord. Crée une application sur le Developer Portal et copie son « Application ID »."
                .to_string(),
        ),
        Err(e) => Err(format!("Échec de connexion : {e}")),
    }
}

pub fn application_name(client_id: &str) -> Option<String> {
    let url = format!("https://discord.com/api/v10/applications/{client_id}/rpc");
    let agent = ureq::AgentBuilder::new()
        .timeout_connect(Duration::from_secs(5))
        .timeout_read(Duration::from_secs(6))
        .user_agent("RoPresence/0.1")
        .build();

    let value: serde_json::Value = agent.get(&url).call().ok()?.into_json().ok()?;
    value
        .get("name")
        .and_then(|n| n.as_str())
        .filter(|s| !s.is_empty())
        .map(String::from)
}
