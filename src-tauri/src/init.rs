use std::sync::mpsc;
use crate::config::{Config, init_logger};
use crate::entities::init_db_coon;
use crate::services::ai::{AI, ONCE_AI};
use tokio::sync::Mutex;
use tracing_appender::non_blocking::WorkerGuard;

///
pub async fn init_app(err_msg: &mut Vec<String>) -> (WorkerGuard, Config, mpsc::Receiver<&'static str>) {
    //todo  不知道这个同步通道在接收消息的时候会不会阻塞tokio的某个线程
    //备选 使用tokio的channel/mpsc::sync_channel()
    let (tx,rx) = mpsc::channel::<&'static str>();
    let log_guard = init_logger(tx);
    let config = Config::load().await;
    init_db_coon().await;
    let mut ai = None;
    if config.ai_config.use_ai {
        let default_ai = config.ai_config.default_ai.clone();
        let o_default_model = config.ai_config.default_model.get(&default_ai);
        if default_ai != "" && o_default_model != None {
            if config.ai_config.keys.contains_key(&default_ai) {
                ai = Some(AI::new(
                    default_ai.clone(),
                    config.ai_config.keys.get(&default_ai).unwrap().clone(),
                    o_default_model.unwrap().to_string(),
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
    (log_guard, config, rx)
}
