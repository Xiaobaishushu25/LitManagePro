use log::{error, info};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::fs;
use std::fs::File;
use std::path::PathBuf;
use std::time::Duration;
use tokio::sync::OnceCell;

use crate::app_errors::AppResult;
use crate::config::CURRENT_DIR;
use crate::entities::table::create_all_need_table;

pub mod doc_and_tag;
pub mod document;
pub mod prelude;
pub mod table;
pub mod tag;
pub mod tag_group;

///实体类，对应数据库中的行数据

pub static DB: OnceCell<DatabaseConnection> = OnceCell::const_new();
pub async fn init_db_coon() {
    let current_dir = CURRENT_DIR.clone();
    let db_path = format!("{}/data/data.db", current_dir);
    let exist = match check_db_file(&db_path, &current_dir) {
        Ok(flag) => flag,
        Err(e) => {
            error!("数据库文件不存在，创建数据库文件{}失败:{:?}", db_path, e);
            panic!("数据库文件不存在，创建数据库文件{}失败:{:?}", db_path, e)
        }
    };
    DB.get_or_init(|| async {
        let url = format!("sqlite:{}?mode=rwc", db_path);
        // url.push_str("?mode=rwc");
        // url.push_str("?mode=rwc");
        let mut opt = ConnectOptions::new(url);
        opt.max_connections(1000)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .sqlx_logging(false);
        let db = Database::connect(opt).await.expect("数据库打开失败");
        if !exist {
            let _ = create_all_need_table(&db).await;
        };
        db
    })
    .await;
}
///打开数据库的日志
#[allow(dead_code)]//function `open_db_log` is never used 这个只有少数需要查看数据库日志时才使用
pub async fn open_db_log() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();
}
pub fn check_db_file(path: &str, current_dir: &str) -> AppResult<bool> {
    if PathBuf::from(path).exists() {
        info!("数据库存在");
        Ok(true)
    } else {
        info!("数据库不存在,创建数据库。");
        let _ = fs::create_dir_all(format!("{}/data", current_dir))?;
        let _ = File::create(path)?;
        Ok(false)
    }
}
