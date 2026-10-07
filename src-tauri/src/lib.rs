use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use reqwest::{Client, Method};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::{AppHandle, Manager};
use tauri_plugin_updater::UpdaterExt;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CraftyRequestArgs {
    url: String,
    method: Option<String>,
    headers: Option<HashMap<String, String>>,
    body_base64: Option<String>,
    insecure_tls: Option<bool>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct CraftyResponse {
    status: u16,
    headers: HashMap<String, String>,
    body_base64: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct UpdateInfo {
    version: String,
    current_version: String,
    notes: Option<String>,
}

fn validate_url(url: &str) -> Result<(), String> {
    let parsed = reqwest::Url::parse(url).map_err(|e| format!("Invalid URL: {e}"))?;
    match parsed.scheme() {
        "http" | "https" => Ok(()),
        other => Err(format!("Unsupported URL scheme: {other}")),
    }
}

fn parse_method(value: Option<String>) -> Result<Method, String> {
    let method = value.unwrap_or_else(|| "GET".to_string());
    Method::from_bytes(method.as_bytes()).map_err(|e| format!("Invalid HTTP method: {e}"))
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct ConnectionConfig {
    crafty_url: String,
    server_id: String,
    api_token: String,
    mods_path: String,
    insecure_tls: bool,
}

fn connection_config_path(app: &AppHandle) -> Result<PathBuf, String> {
    let dir = app
        .path()
        .app_config_dir()
        .map_err(|e| format!("Could not resolve Tinker config folder: {e}"))?;
    std::fs::create_dir_all(&dir)
        .map_err(|e| format!("Could not create Tinker config folder: {e}"))?;
    Ok(dir.join("connection.json"))
}

#[tauri::command]
fn load_connection_config(app: AppHandle) -> Result<Option<ConnectionConfig>, String> {
    let path = connection_config_path(&app)?;
    match std::fs::read_to_string(&path) {
        Ok(raw) => {
            let config = serde_json::from_str::<ConnectionConfig>(&raw)
                .map_err(|e| format!("Could not read saved Crafty connection: {e}"))?;
            Ok(Some(config))
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(format!("Could not read saved Crafty connection: {e}")),
    }
}

#[tauri::command]
fn save_connection_config(app: AppHandle, config: ConnectionConfig) -> Result<(), String> {
    let path = connection_config_path(&app)?;
    let raw = serde_json::to_string_pretty(&config)
        .map_err(|e| format!("Could not encode Crafty connection: {e}"))?;
    std::fs::write(&path, raw)
        .map_err(|e| format!("Could not save Crafty connection: {e}"))?;
    Ok(())
}

#[tauri::command]
async fn crafty_request(args: CraftyRequestArgs) -> Result<CraftyResponse, String> {
    validate_url(&args.url)?;

    let builder = Client::builder()
        .user_agent("Tinker-MC/0.2")
        .connect_timeout(std::time::Duration::from_secs(8))
        .timeout(std::time::Duration::from_secs(90));

    let client = if args.insecure_tls.unwrap_or(false) {
        builder.danger_accept_invalid_certs(true)
    } else {
        builder
    }
    .build()
    .map_err(|e| format!("Could not create HTTP client: {e}"))?;

    let method = parse_method(args.method)?;
    let mut request = client.request(method, &args.url);

    if let Some(headers) = args.headers {
        for (name, value) in headers {
            request = request.header(name, value);
        }
    }

    if let Some(encoded) = args.body_base64 {
        let body = BASE64
            .decode(encoded)
            .map_err(|e| format!("Invalid request body encoding: {e}"))?;
        request = request.body(body);
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Crafty request failed: {e}"))?;

    let status = response.status().as_u16();
    let mut headers = HashMap::new();
    for (name, value) in response.headers() {
        if let Ok(value) = value.to_str() {
            headers.insert(name.to_string(), value.to_string());
        }
    }

    let body = response
        .bytes()
        .await
        .map_err(|e| format!("Could not read Crafty response: {e}"))?;

    Ok(CraftyResponse {
        status,
        headers,
        body_base64: BASE64.encode(body),
    })
}

#[tauri::command]
async fn ping() -> Result<String, String> {
    Ok("Tinker native bridge is running".to_string())
}

#[tauri::command]
async fn check_for_update(app: AppHandle) -> Result<Option<UpdateInfo>, String> {
    let update = app
        .updater()
        .map_err(|e| e.to_string())?
        .check()
        .await
        .map_err(|e| e.to_string())?;

    Ok(update.map(|u| UpdateInfo {
        version: u.version,
        current_version: u.current_version,
        notes: u.body,
    }))
}

#[tauri::command]
async fn install_update(app: AppHandle) -> Result<(), String> {
    let update = app
        .updater()
        .map_err(|e| e.to_string())?
        .check()
        .await
        .map_err(|e| e.to_string())?;

    let Some(update) = update else {
        return Err("No update is currently available.".to_string());
    };

    update
        .download_and_install(|_, _| {}, || {})
        .await
        .map_err(|e| e.to_string())?;

    // The Windows updater exits the current process after successfully launching
    // the installer. macOS/Linux need an explicit relaunch after install.
    if !cfg!(target_os = "windows") {
        app.restart();
    }

    Ok(())
}

#[cfg(target_os = "windows")]
fn likely_nbt_explorer_paths() -> Vec<PathBuf> {
    let mut paths = Vec::new();
    for var in ["PROGRAMFILES", "PROGRAMFILES(X86)", "LOCALAPPDATA"] {
        if let Ok(root) = std::env::var(var) {
            let root = PathBuf::from(root);
            paths.push(root.join(r"NBTExplorer\NBTExplorer.exe"));
            paths.push(root.join(r"Programs\NBTExplorer\NBTExplorer.exe"));
        }
    }
    paths
}

#[tauri::command]
fn open_external_url(url: String) -> Result<(), String> {
    #[cfg(not(target_os = "windows"))]
    {
        let _ = url;
        return Err("External URL opening is currently implemented for Windows builds.".to_string());
    }

    #[cfg(target_os = "windows")]
    {
        let safe = url.trim();
        if !(safe.starts_with("https://") || safe.starts_with("http://")) {
            return Err("Only http:// and https:// URLs can be opened.".to_string());
        }
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        std::process::Command::new("rundll32.exe")
            .args(["url.dll,FileProtocolHandler", safe])
            .creation_flags(CREATE_NO_WINDOW)
            .spawn()
            .map_err(|e| format!("Could not open the web browser: {e}"))?;
        Ok(())
    }
}

#[tauri::command]
async fn launch_nbt_explorer(_app: AppHandle, path: String, data_base64: Option<String>) -> Result<(), String> {
    #[cfg(not(target_os = "windows"))]
    {
        let _ = path;
        let _ = data_base64;
        return Err("The NBT Explorer launcher is currently Windows-only.".to_string());
    }

    #[cfg(target_os = "windows")]
    {
        let encoded = data_base64.ok_or_else(|| "Player-data bytes were not provided.".to_string())?;
        let data = BASE64
            .decode(encoded)
            .map_err(|e| format!("Could not decode player data: {e}"))?;

        let file_stem = PathBuf::from(&path)
            .file_stem()
            .and_then(|n| n.to_str())
            .unwrap_or("player")
            .chars()
            .map(|c| if c.is_ascii_alphanumeric() || c == '-' || c == '_' { c } else { '_' })
            .collect::<String>();

        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| format!("Could not get system time: {e}"))?
            .as_millis();

        let temp_root = std::env::temp_dir().join("Tinker").join("playerdata");
        std::fs::create_dir_all(&temp_root)
            .map_err(|e| format!("Could not create Tinker player-data temp folder: {e}"))?;

        let local_path = temp_root.join(format!("{}-{}.dat", file_stem, stamp));
        std::fs::write(&local_path, data)
            .map_err(|e| format!("Could not write local player-data copy: {e}"))?;

        for exe in likely_nbt_explorer_paths() {
            if exe.is_file() {
                std::process::Command::new(&exe)
                    .arg(&local_path)
                    .spawn()
                    .map_err(|e| format!("Could not start NBT Explorer: {e}"))?;
                return Ok(());
            }
        }

        Err("NBT Explorer was not found in the usual Windows install locations. Install NBT Explorer or choose another player-data editor.".to_string())
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            crafty_request,
            ping,
            load_connection_config,
            save_connection_config,
            check_for_update,
            install_update,
            launch_nbt_explorer,
            open_external_url
        ])
        .run(tauri::generate_context!())
        .expect("error while running Tinker");
}
