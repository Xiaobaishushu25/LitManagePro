use std::process::exit;
use std::sync::Mutex;
use tracing::info;
use crate::init::init_app;
use crate::services::commands::tag::{query_tag_groups, create_tag_group, rename_tag_group, create_tag, delete_tag, delete_group, insert_doc_and_tag, delete_doc_and_tag};
use crate::services::commands::config::{get_config,save_config};
use crate::services::commands::doc::{insert_docs,query_docs_by_tags};

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
            delete_tag,
            create_tag_group,
            rename_tag_group,
            delete_group,
            get_config,
            save_config,
            insert_doc_and_tag,
            delete_doc_and_tag,
            insert_docs,
            query_docs_by_tags,
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