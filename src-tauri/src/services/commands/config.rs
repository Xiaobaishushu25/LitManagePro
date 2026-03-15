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
    let mut mutex_guard = state.lock().map_err(|e| format!("获取配置锁失败：{}", e))?; // 替换unwrap，返回具体错误
    // 1. 先获取旧的AI配置（更新全局配置前先保存）
    let old_ai_config = mutex_guard.ai_config.clone();
    // 2. 更新全局配置
    mutex_guard.update(config.clone());
    let new_ai_config = config.ai_config;
    // 核心：仅当开启AI且新旧配置不一致时，才执行AI更新逻辑
    if new_ai_config.use_ai && new_ai_config != old_ai_config {
        tokio::spawn(async move {
            // 获取全局AI实例（优化unwrap为更安全的处理）
            let Some(once_ai) = ONCE_AI.get() else {
                error!("ONCE_AI 未初始化");
                let _ = app_handle.emit("backend_message", "AI实例存储未初始化");
                return;
            };
            let mut o_ai = once_ai.lock().await;
            let result = if let Some(ai) = &mut *o_ai {
                // 已有AI实例，更新配置
                ai.update_by_ai_config(&new_ai_config)
            } else {
                // 无AI实例，创建新实例
                match AI::get_ai_from_ai_config(&new_ai_config) {
                    Ok(ai) => {
                        info!("创建新AI实例：{}", ai);
                        *o_ai = Some(ai);
                        Ok(())
                    }
                    Err(e) => Err(e),
                }
            };
            // 错误处理
            if let Err(e) = result {
                error!("更新AI失败：{}", e);
                let _ = app_handle.emit(
                    "backend_message",
                    format!("根据配置更新AI失败：{}", e)
                );
            }
        });
    }

    Ok(())
}
// #[tauri::command]
// pub fn update_config(
//     app_handle: AppHandle,
//     state: State<'_, Mutex<Config>>,
//     config: Config,
// ) -> Result<(), String> {
//     info!("更新配置：{:?}", config);
//     let mut mutex_guard = state.lock().unwrap();
//     mutex_guard.update(config.clone());
//     let ai_config = config.ai_config;
//     if ai_config.use_ai {
//         tokio::spawn(async move {
//             let o_ai = &mut *ONCE_AI.get().unwrap().lock().await;
//             let result = if let Some(ai) = o_ai {
//                 ai.update_by_ai_config(&ai_config)
//             } else {
//                 match AI::get_ai_from_ai_config(&ai_config) {
//                     Ok(ai) => {
//                         info!("更新AI：{}", ai);
//                         *o_ai = Some(ai);
//                         Ok(())
//                     }
//                     Err(e) => Err(e),
//                 }
//             };
//             if let Err(e) = result {
//                 error!("更新AI失败：{}", e);
//                 let _ = app_handle.emit("backend_message", "根据配置更新AI失败。");
//             }
//         });
//     }
//     Ok(())
// }
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
