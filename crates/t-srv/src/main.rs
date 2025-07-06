// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Result;

fn main() -> Result<()> {
    t_srv::log::init_tracing_subscriber_log();
    t_srv::run()
}
