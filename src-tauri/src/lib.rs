pub mod log;
pub mod rpc;

use crate::log::{error, info, trace};
use tauri::{async_runtime::spawn, Result};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
#[log::instrument]
fn greet(name: &str) -> String {
    trace!("Greet");
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> Result<()> {
    tauri::Builder::default()
        // .plugin(crate::log::init_tauri_log_plugin())
        .plugin(tauri_plugin_opener::init())
        .setup(|_| {
            info!("tauri setup");
            spawn(async {
                if let Err(err) = rpc::run().await {
                    error!("rpc run failed: {err}");
                }
                info!("rpc finish");
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
}
