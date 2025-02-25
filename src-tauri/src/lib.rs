use std::process::exit;
use std::sync::Mutex;
use tracing::info;
use crate::init::init_app;
use crate::services::commands::tag::{query_tag_groups, create_tag_group, update_tag_group_name, create_tag,};
use crate::services::commands::config::{get_config,save_config};

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
            query_tag_groups,
            create_tag,
            create_tag_group,
            update_tag_group_name,
            get_config,
            save_config,
            exit_app
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
#[tauri::command]
async fn exit_app()->Result<(),()> {
    info!("退出程序");
    // sleep(Duration::from_secs(1)).await;
    exit(0)
}