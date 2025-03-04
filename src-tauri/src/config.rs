use crate::app_errors::AppResult;
use crate::services::util::file::get_and_save_icon;
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::sync::LazyLock;
use std::{env, fs, io};
use time::UtcOffset;
use time::macros::format_description;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::rolling::RollingFileAppender;
use tracing_subscriber::fmt::time::OffsetTime;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Layer, Registry, fmt};
use crate::app_errors::AppError::Tip;

pub static CURRENT_DIR: LazyLock<String> = LazyLock::new(|| {
    let current_dir = &env::current_dir().expect("无法获取当前目录");
    current_dir.to_string_lossy().to_string()
});

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    ui_config: UiConfig,
    pub ai_config: AiConfig,
    pub exe_configs: Vec<ExeConfig>,
}

impl Config {
    /**
     * 加载配置文件
     */
    pub async fn load() -> Self {
        info!("load config...");
        let path = format!("{}/data/config", CURRENT_DIR.clone());
        match check_config_file(&path, &CURRENT_DIR.clone()) {
            Ok(config) => {
                info!("load config success{:?}", config);
                config
            }
            Err(e) => {
                panic!("创建或解析配置文件{}失败:{:?}", path, e)
            }
        }
    }
    pub async fn update(&mut self, config: Config) {
        *self = config;
    }
    /**
     * 保存配置文件
     */
    pub async fn save_to_file(&self) -> AppResult<()> {
        let path = format!("{}/data/config", CURRENT_DIR.clone());
        let mut config_file = OpenOptions::new()
            .write(true) // 以写入模式打开文件
            .truncate(true) // 清空文件内容
            .open(path)?;
        config_file.write_all(serde_json::to_string(self)?.as_bytes())?;
        Ok(())
    }
}
impl Default for Config {
    fn default() -> Self {
        Config {
            ui_config: UiConfig::default(),
            ai_config: AiConfig::default(),
            exe_configs: vec![],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    //tag组是否打开，key为tag_group_name，value为bool
    tag_group_state: HashMap<i32, bool>,
    save_tag_groups: Vec<Vec<i32>>,
    //表格是否展开总结行(在有总结的情况时)
    table_expand: bool,
}
impl Default for UiConfig {
    fn default() -> Self {
        UiConfig {
            tag_group_state: HashMap::new(),
            save_tag_groups: vec![],
            table_expand: true,
        }
    }
}

///executable program的配置
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExeConfig {
    pub name: String,
    pub path: String,
    pub icon_path: String,
}

impl ExeConfig {
    pub fn new(_path: &str) -> AppResult<Self> {
        let (name, icon_path) = get_and_save_icon(_path, 34).map_err(|e| {Tip(format!("获取程序图标出错{:#}", e))})?;
        // let path = Path::new(_path);
        // let icon_dir = format!("{}/icon", CURRENT_DIR.clone());
        // let name = path.file_stem().unwrap().to_str().unwrap().to_string();
        Ok(ExeConfig {
            name,
            path: _path.to_string(),
            icon_path,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiConfig {
    pub(crate) use_ai: bool,
    //默认使用的ai，分别为：kimi,deepseek
    pub default_ai: String,
    //默认使用的模型
    pub default_model: String,
    //模型，key为ai名称，value为模型名称集合
    pub models: HashMap<String, Vec<String>>,
    //key为ai名称，value为ai的key
    pub keys: HashMap<String, String>,
    pub online: bool,
}
impl Default for AiConfig {
    fn default() -> Self {
        AiConfig {
            use_ai: false,
            default_ai: "kimi".to_string(),
            default_model: "moonshot-v1-8k".to_string(),
            models: HashMap::from([("kimi".to_string(), vec!["moonshot-v1-8k".to_string()])]),
            keys: HashMap::new(),
            // keys:HashMap::new(),
            online: false,
        }
    }
}

fn check_config_file(path: &str, current_dir: &str) -> AppResult<Config> {
    let mut config_file: File = if PathBuf::from(path).exists() {
        info!("配置存在");
        if let Ok(config) = serde_json::from_str::<Config>(&fs::read_to_string(path)?) {
            return Ok(config); //如果正确解析配置文件，直接返回
        } else {
            error!("配置文件格式错误，将重新创建配置文件。");
            //清除配置文件内容
            // 打开文件并清空内容
            OpenOptions::new()
                .write(true) // 以写入模式打开文件
                .truncate(true) // 清空文件内容
                .open(path)?
        }
    } else {
        info!("配置不存在,创建配置。");
        fs::create_dir_all(format!("{}/data", current_dir))?;
        File::create(path)?
    };
    //如果上面正确读取配置文件就已经返回了，到这里说明配置文件没有内容，需要初始化默认配置
    // let config = Config::init_default();
    let config = Config::default();
    let config_string = serde_json::to_string(&config)?;
    config_file.write_all(config_string.as_bytes())?;
    Ok(config)
}

/// 初始化日志
pub fn init_logger() -> WorkerGuard {
    // 配置文件日志
    let log_path = format!("{}/data/log", CURRENT_DIR.clone());
    fs::create_dir_all(&log_path).expect("无法创建日志目录");

    let local_time = OffsetTime::new(
        UtcOffset::from_hms(8, 0, 0).unwrap(),
        format_description!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]"),
    );

    let file_appender = RollingFileAppender::builder()
        .filename_prefix("litManagePro") //意味着生成的日志文件名会以 "litManagePro" 开头。
        .filename_suffix("log") //生成的日志文件名会以 .log 结尾
        .build(log_path)
        .expect("无法初始化滚动文件追加器");

    let (non_blocking_file, worker_guard) = tracing_appender::non_blocking(file_appender);
    let file_layer = fmt::layer()
        .with_writer(non_blocking_file)
        .with_ansi(false) //表示不使用 ANSI 转义码。这通常用于文件日志，因为文件通常不支持 ANSI 转义码（如颜色、样式等）。
        .with_line_number(true) //表示在日志中包含行号。这有助于调试时快速定位日志的来源。
        .with_target(true) //表示在日志中包含目标。目标通常是一个字符串，用于标识日志的来源，例如模块名或函数名。
        // .with_thread_ids(true)//表示在日志中包含线程 ID。这有助于区分不同线程的日志，特别是在多线程环境中。
        .with_level(true) //表示在日志中包含日志级别（如 INFO、ERROR 等）。这有助于快速识别日志的严重性。
        .with_thread_names(true)
        .with_timer(local_time.clone())
        .with_filter(EnvFilter::new("error")); //文件只显示错误级别的日志

    // 配置控制台日志
    let console_layer = fmt::layer()
        .with_writer(io::stdout)
        .with_ansi(true)
        .with_line_number(true)
        .with_target(true)
        // .with_thread_ids(true)
        .with_level(true)
        // .with_thread_names(true)
        .with_timer(local_time)
        .with_filter(EnvFilter::new(
            "info,tao::platform_impl::platform::event_loop::runner=error",
        ));
    // .with_filter(EnvFilter::new("info")); // 控制台显示 info 级别及以上的日志

    // 配置日志订阅器
    Registry::default()
        .with(console_layer)
        .with(file_layer)
        .with(EnvFilter::new("info"))
        .init();

    // tracing::subscriber::set_global_default(subscriber)
    //     .expect("设置日志订阅器失败");
    worker_guard
}
#[cfg(test)]
mod test {
    use crate::config::ExeConfig;

    #[test]
    fn test_new_exe() {
        let exe_config = ExeConfig::new("D:\\知云\\ZhiyunTranslator\\ZhiYunTranslator.exe");
        println!("{:?}", exe_config);
        // println!("{}", serde_json::to_string(&exe_config).unwrap());
    }
}
