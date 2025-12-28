use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::State;

const DISCORD_APP_ID: &str = "1454908743140511786";

struct DiscordClient {
    client: Mutex<Option<DiscordIpcClient>>,
    start_time: Mutex<Option<i64>>,
}

impl Default for DiscordClient {
    fn default() -> Self {
        Self {
            client: Mutex::new(None),
            start_time: Mutex::new(None),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivityConfig {
    pub activity_type: String,
    pub custom_activity: Option<String>,
    pub server_name: Option<String>,
    pub show_elapsed: bool,
    pub on_server: bool,
    pub party_size: i32,
    pub party_max: i32,
    pub join_server_url: Option<String>,
    pub show_orbis_button: bool,
}

#[tauri::command]
fn connect_discord(discord: State<DiscordClient>) -> Result<String, String> {
    let mut client_guard = discord.client.lock().map_err(|e| e.to_string())?;
    
    if client_guard.is_some() {
        return Ok("Already connected".to_string());
    }

    let mut client = DiscordIpcClient::new(DISCORD_APP_ID);
    client.connect().map_err(|e| format!("Failed to connect to Discord: {}", e))?;

    let start_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    
    *discord.start_time.lock().map_err(|e| e.to_string())? = Some(start_time);
    *client_guard = Some(client);

    Ok("Connected to Discord".to_string())
}

#[tauri::command]
fn disconnect_discord(discord: State<DiscordClient>) -> Result<String, String> {
    let mut client_guard = discord.client.lock().map_err(|e| e.to_string())?;

    if let Some(mut client) = client_guard.take() {
        client.close().map_err(|e| format!("Failed to disconnect: {}", e))?;
        *discord.start_time.lock().map_err(|e| e.to_string())? = None;
        Ok("Disconnected from Discord".to_string())
    } else {
        Ok("Not connected".to_string())
    }
}

#[tauri::command]
fn update_activity(
    config: ActivityConfig,
    discord: State<DiscordClient>,
) -> Result<String, String> {
    let mut client_guard = discord.client.lock().map_err(|e| e.to_string())?;

    let client = client_guard
        .as_mut()
        .ok_or("Not connected to Discord")?;

    let details = match config.activity_type.as_str() {
        "exploring" => "Exploring the Alterverse",
        "building" => "Building in Creative Mode",
        "pvp" => "Fighting in PvP Arena",
        "dungeon" => "Raiding a Dungeon",
        "taming" => "Taming Creatures",
        "crafting" => "Crafting Items",
        "adventure" => "Adventuring with Friends",
        "custom" => config.custom_activity.as_deref().filter(|s| !s.is_empty()).unwrap_or("Playing Hytale"),
        _ => "Playing Hytale",
    };

    let state = config
        .server_name
        .as_ref()
        .filter(|s| !s.is_empty())
        .map(|s| format!("Playing on {}", s));

    let start_time = if config.show_elapsed {
        discord.start_time.lock().ok().and_then(|g| *g)
    } else {
        None
    };

    let party_id = format!("hytale-party-{}", uuid::Uuid::new_v4());

    let mut buttons = Vec::new();
    
    let join_url_owned = config.join_server_url
        .map(|url| {
            let url = url.trim().to_string();
            if url.is_empty() {
                url
            } else if !url.starts_with("http://") && !url.starts_with("https://") {
                format!("https://{}", url)
            } else {
                url
            }
        })
        .unwrap_or_default();
    
    if !join_url_owned.is_empty() {
        buttons.push(activity::Button::new("Join Server", &join_url_owned));
    }
    
    if config.show_orbis_button {
        buttons.push(activity::Button::new("Discover more on Orbis", "https://orbis.place"));
    }

    let mut activity_builder = activity::Activity::new()
        .details(details)
        .assets(
            activity::Assets::new()
                .large_image("hytale_logo")
                .large_text("Hytale")
                .small_image(&config.activity_type)
                .small_text(details),
        )
        .buttons(buttons);

    if let Some(ts) = start_time {
        activity_builder = activity_builder.timestamps(activity::Timestamps::new().start(ts));
    }

    if config.on_server {
        if let Some(ref s) = state {
            activity_builder = activity_builder.state(s);
        }
        activity_builder = activity_builder.party(
            activity::Party::new()
                .id(&party_id)
                .size([config.party_size, config.party_max]),
        );
    }

    client
        .set_activity(activity_builder)
        .map_err(|e| format!("Failed to set activity: {}", e))?;

    Ok("Activity updated".to_string())
}

#[tauri::command]
fn get_connection_status(discord: State<DiscordClient>) -> bool {
    discord
        .client
        .lock()
        .map(|g| g.is_some())
        .unwrap_or(false)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(DiscordClient::default())
        .invoke_handler(tauri::generate_handler![
            connect_discord,
            disconnect_discord,
            update_activity,
            get_connection_status
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
