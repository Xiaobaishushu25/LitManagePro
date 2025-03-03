use crate::config::{Config, init_logger};
use crate::entities::init_db_coon;
use crate::services::ai::{AI, ONCE_AI};
use tokio::sync::Mutex;
use tracing_appender::non_blocking::WorkerGuard;

pub async fn init_app(err_msg: &mut Vec<String>) -> (WorkerGuard, Config) {
    let log_guard = init_logger();
    let config = Config::load().await;
    init_db_coon().await;
    let mut ai = None;
    if config.ai_config.use_ai {
        if config.ai_config.default_ai != "" && config.ai_config.default_model != "" {
            if config
                .ai_config
                .keys
                .contains_key(&config.ai_config.default_ai)
            {
                ai = Some(AI::new(
                    config.ai_config.default_ai.clone(),
                    config
                        .ai_config
                        .keys
                        .get(&config.ai_config.default_ai)
                        .unwrap()
                        .clone(),
                    config.ai_config.default_model.clone(),
                    config.ai_config.online,
                ));
            } else {
                err_msg.push("请配置ai的key".to_string());
            }
        } else {
            err_msg.push("请配置ai以及默认模型".to_string());
        }
    }
    ONCE_AI.set(Mutex::new(ai)).expect("ONCE_AI set error");
    (log_guard, config)
}
