use std::sync::Mutex;
use log::{error, info};
use crate::init::init_app;
use crate::services::commands::tag::query_tags;

mod init;
mod services;

pub mod app_errors;
pub mod config;
mod entities;
mod dtos;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    let (_log_guard,config) = init_app().await;
    info!("litManagePro ui start...");
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_os::init())
        .manage(Mutex::new(config))
        .invoke_handler(tauri::generate_handler![
            query_tags,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
