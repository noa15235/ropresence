//! Discord Rich Presence over the official local IPC socket.
//!
//! We only ever talk to the already-running Discord desktop client through its
//! local named pipe (`discord-ipc-0`). No token, no bot, no account access —
//! this is the officially supported Rich Presence usage.

pub mod reconnect;
pub mod rpc;

pub use reconnect::Backoff;
pub use rpc::{ActivityPayload, DiscordManager};

use discord_rich_presence::activity::Activity;
use discord_rich_presence::{DiscordIpc, DiscordIpcClient};
use std::time::Duration;

/// Definitively test a Client ID by actually opening an RPC connection and
/// pushing a probe activity. This is the only reliable way to tell a real
/// application id from a non-existent one (Discord accepts the handshake either
/// way, but closes the pipe on `set_activity` for an invalid app).
///
/// Returns the app name (possibly empty) on success, or a clear French error.
pub fn test_connection(client_id: &str) -> Result<String, String> {
    let mut client = DiscordIpcClient::new(client_id)
        .map_err(|e| format!("Client ID invalide : {e}"))?;
    client.connect().map_err(|_| {
        "Discord est introuvable. Ouvre l'application Discord puis réessaie.".to_string()
    })?;
    // Let the handshake settle before the probe.
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

/// Best-effort lookup of a Discord application's public name from its id.
///
/// Uses the public RPC metadata endpoint, which only returns data for apps that
/// expose an RPC profile — so a 404 here does NOT mean the id is invalid (many
/// valid presence apps return 404). Therefore this never decides validity; it
/// only enriches the UI with a name when one is available.
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
