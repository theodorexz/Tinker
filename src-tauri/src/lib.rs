use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use reqwest::{Client, Method};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tauri::AppHandle;
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

#[tauri::command]
async fn crafty_request(args: CraftyRequestArgs) -> Result<CraftyResponse, String> {
    validate_url(&args.url)?;

    let builder = Client::builder()
        .user_agent("Tinker/0.1")
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
        .map_err(|e| format!("Tinker HTTP request failed: {e}"))?;

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
        .map_err(|e| format!("Could not read HTTP response: {e}"))?;

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
    let updater = app
        .updater()
        .map_err(|e| format!("Updater could not start: {e}"))?;

    let update = updater
        .check()
        .await
        .map_err(|e| format!("Update check failed: {e}"))?;

    Ok(update.map(|u| UpdateInfo {
        version: u.version,
        current_version: u.current_version,
        notes: u.body,
    }))
}

#[tauri::command]
async fn install_update(app: AppHandle) -> Result<(), String> {
    let updater = app
        .updater()
        .map_err(|e| format!("Updater could not start: {e}"))?;

    let update = updater
        .check()
        .await
        .map_err(|e| format!("Update check failed: {e}"))?;

    let Some(update) = update else {
        return Err("No update is currently available.".to_string());
    };

    // Tauri's updater verifies the signed bundle before installing it.
    // On Windows, download_and_install launches the installer and exits the app.
    update
        .download_and_install(|_, _| {}, || {})
        .await
        .map_err(|e| format!("Update installation failed: {e}"))?;

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            crafty_request,
            ping,
            check_for_update,
            install_update
        ])
        .run(tauri::generate_context!())
        .expect("error while running Tinker");
}
