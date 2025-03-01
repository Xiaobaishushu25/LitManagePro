use crate::config::Config;
use log::{error, info};
use std::sync::Mutex;
use tauri::State;

#[tauri::command]
pub async fn get_config(state: State<'_, Mutex<Config>>) -> Result<Config, String> {
    let mutex_guard = state.lock().unwrap();
    Ok((*mutex_guard).clone())
}
#[tauri::command]
pub async fn save_config(config: Config) -> Result<(), String> {
    info!("保存配置：{:?}", config);
    match config.save_to_file().await {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("保存配置失败：{:?}", e);
            Err("保存配置失败".to_string())
        }
    }
}
