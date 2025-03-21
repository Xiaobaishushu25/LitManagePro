use crate::config::{Config, ExeConfig};
use crate::services::ai::{AI, ONCE_AI};
use log::{error, info};
use std::sync::Mutex;
use tauri::{AppHandle, Emitter, State};

#[tauri::command]
pub async fn get_config(state: State<'_, Mutex<Config>>) -> Result<Config, String> {
    let mutex_guard = state.lock().unwrap();
    Ok((*mutex_guard).clone())
}
#[tauri::command]
pub fn update_config(
    app_handle: AppHandle,
    state: State<'_, Mutex<Config>>,
    config: Config,
) -> Result<(), String> {
    info!("更新配置：{:?}", config);
    let mut mutex_guard = state.lock().unwrap();
    mutex_guard.update(config.clone());
    let ai_config = config.ai_config;
    if ai_config.use_ai {
        tokio::spawn(async move {
            let o_ai = &mut *ONCE_AI.get().unwrap().lock().await;
            let result = if let Some(ai) = o_ai {
                ai.update_by_ai_config(&ai_config)
            } else {
                match AI::get_ai_from_ai_config(&ai_config) {
                    Ok(ai) => {
                        info!("更新AI：{}", ai);
                        *o_ai = Some(ai);
                        Ok(())
                    }
                    Err(e) => Err(e),
                }
            };
            if let Err(e) = result {
                error!("更新AI失败：{}", e);
                let _ = app_handle.emit("backend_message", "根据配置更新AI失败。");
            }
        });
    }
    Ok(())
}
#[tauri::command]
pub async fn save_config(config: Config) -> Result<(), String> {
    info!("保存配置：{:?}", config);
    match config.save_to_file().await {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("保存配置失败：{}", e);
            Err("保存配置失败".to_string())
        }
    }
}
#[tauri::command]
pub async fn add_new_exe(path: String) -> Result<ExeConfig, String> {
    match ExeConfig::new(&path) {
        Ok(exe_config) => {
            info!("添加成功：{:?}", exe_config);
            Ok(exe_config)
        }
        Err(e) => {
            error!("添加程序失败：{}", e);
            Err("添加程序失败".to_string())
        }
    }
}
