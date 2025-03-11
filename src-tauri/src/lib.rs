use crate::init::init_app;
use crate::services::commands::config::{get_config, save_config,add_new_exe,update_config};
use crate::services::commands::doc::{
    delete_docs, insert_docs, open_dir, open_by_system, open_by_app, open_with_exe, query_docs_by_tags,
    update_doc_detail, summarize_docs_by_ai,
};
use crate::services::commands::tag::{
    create_tag, create_tag_group, delete_doc_tag, delete_group, delete_tag, insert_doc_tag,
    query_tag_groups, rename_tag_group, update_doc_tags,
};
use std::process::exit;
use std::sync::Mutex;
use tauri::State;
use tracing::info;

mod init;
mod services;

pub mod app_errors;
pub mod config;
mod dtos;
mod entities;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    //用于存储界面初始化之前的错误信息
    let mut err_msg = vec![];
    //_log_guard存活的周期内才能写入日志，所以需要返回给调用者。
    let (_log_guard, config) = init_app(&mut err_msg).await;
    info!("litManagePro ui start...");
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_window_state::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_os::init())
        .manage(Mutex::new(config))
        .manage(err_msg)
        .invoke_handler(tauri::generate_handler![
            first_run,
            query_tag_groups,
            create_tag,
            delete_tag,
            create_tag_group,
            rename_tag_group,
            delete_group,
            get_config,
            save_config,
            update_config,
            add_new_exe,
            insert_doc_tag,
            delete_doc_tag,
            update_doc_tags,
            insert_docs,
            query_docs_by_tags,
            update_doc_detail,
            summarize_docs_by_ai,
            delete_doc_tag,
            delete_docs,
            open_dir,
            open_by_system,
            open_by_app,
            open_with_exe,
            exit_app
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
#[tauri::command]
async fn first_run(state: State<'_, Vec<String>>) -> Result<Vec<String>, ()> {
    Ok(state.inner().clone())
}
#[tauri::command]
async fn exit_app() -> Result<(), ()> {
    info!("退出程序");
    // sleep(Duration::from_secs(1)).await;
    exit(0)
}
