use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};
use serde::Deserialize;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

static SHUTDOWN: AtomicBool = AtomicBool::new(false);

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct DiscordPresenceConfig {
    pub enabled: bool,
    pub client_id: String,
    /// Pierwsza linia pod nazwą aplikacji (np. „Rozmawia z innymi”).
    pub details: String,
    /// Druga linia (np. „app.klovy.chat”).
    pub state: String,
    pub large_image_key: String,
    pub large_image_text: String,
    /// Licznik czasu od uruchomienia apki (jak u Stoat).
    pub show_elapsed: bool,
}

impl Default for DiscordPresenceConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            client_id: String::new(),
            details: "Chatting with others".to_string(),
            state: "app.klovy.chat".to_string(),
            large_image_key: "klovy_logo".to_string(),
            large_image_text: "Klovy Chat".to_string(),
            show_elapsed: true,
        }
    }
}

fn unix_now_secs() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs() as i64)
        .unwrap_or(0)
}

fn build_activity(config: &DiscordPresenceConfig, started_at: i64) -> activity::Activity<'_> {
    let mut assets = activity::Assets::new();
    if !config.large_image_key.is_empty() {
        assets = assets.large_image(config.large_image_key.clone());
    }
    if !config.large_image_text.is_empty() {
        assets = assets.large_text(config.large_image_text.clone());
    }

    let mut activity = activity::Activity::new().assets(assets);
    if !config.details.is_empty() {
        activity = activity.details(config.details.clone());
    }
    if !config.state.is_empty() {
        activity = activity.state(config.state.clone());
    }
    if config.show_elapsed {
        activity = activity.timestamps(activity::Timestamps::new().start(started_at));
    }
    activity
}

pub fn init_from_app_config(app_config: &tauri::Config) {
    let config = app_config
        .plugins
        .0
        .get("discordPresence")
        .and_then(|value| serde_json::from_value(value.clone()).ok())
        .unwrap_or_default();
    init_from_config(config);
}

pub fn init_from_config(config: DiscordPresenceConfig) {
    if !config.enabled || config.client_id.trim().is_empty() {
        return;
    }

    SHUTDOWN.store(false, Ordering::SeqCst);
    thread::spawn(move || run_presence_loop(config));
}

fn run_presence_loop(config: DiscordPresenceConfig) {
    let mut client = DiscordIpcClient::new(config.client_id.trim());
    let started_at = unix_now_secs();

    if client.connect().is_err() {
        return;
    }

    while !SHUTDOWN.load(Ordering::SeqCst) {
        let activity = build_activity(&config, started_at);
        if client.set_activity(activity).is_err() {
            break;
        }

        for _ in 0..15 {
            if SHUTDOWN.load(Ordering::SeqCst) {
                break;
            }
            thread::sleep(Duration::from_secs(1));
        }
    }

    let _ = client.close();
}

pub fn stop() {
    SHUTDOWN.store(true, Ordering::SeqCst);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_is_disabled_without_client_id() {
        let config = DiscordPresenceConfig::default();
        assert!(config.client_id.is_empty());
    }
}
