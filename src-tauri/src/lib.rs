mod log;
mod rpc;

use crate::log::{error, info, init_tauri_log_plugin};
use tauri::async_runtime::spawn;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    info!("greet: {name}");
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    tauri::Builder::default()
        .plugin(init_tauri_log_plugin())
        .plugin(tauri_plugin_opener::init())
        .setup(|_| {
            info!("tauri setup");
            spawn(async {
                if let Err(err) = rpc::run().await {
                    error!("rpc run failed: {err}");
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())?;

    rpc::run().await?;

    Ok(())
}
