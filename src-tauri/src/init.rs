use tracing_appender::non_blocking::WorkerGuard;
use crate::config::{init_logger, Config};
use crate::entities::init_db_coon;

pub async fn init_app() -> (WorkerGuard, Config){
    let log_guard = init_logger();
    let config = Config::load().await;
    init_db_coon().await;
    (log_guard,config)
}

