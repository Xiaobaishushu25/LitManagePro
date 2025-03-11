// use std::backtrace::Backtrace;
use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("{0}")]
    Tip(String),
    #[error("error:`{0}`")]
    AnyHow(#[from] anyhow::Error),
    #[error("io::Error:`{0}`")]
    IoError(#[from] io::Error),
    #[error("serde_json::Error:`{0}`")]
    Serde(#[from] serde_json::Error),
    #[error("reqwest::Error:`{0}`")]
    Http(#[from] reqwest::Error),
    // HttpError{#[from] source:reqwest::Error,backtrace: Backtrace},
    //数据库错误
    #[error("sea_orm::DbErr:`{0}`")]
    Sqlx(#[from] sea_orm::DbErr),
    #[error("提取pdf内容出错:`{0}`")]
    Extract(#[from] pdf_extract::OutputError), // #[error("chrono::ParseError:`{0}`")]
                                               // ParseError(#[from] chrono::ParseError)
                                               // #[error("http::ParseError:`{0}`")]
                                               // ParseError(#[from] ParseError),
                                               // #[error("sea_orm::DbErr:Error:`{0}`")]
                                               // DbErr(#[from] sea_orm::DbErr),
}
pub type AppResult<T> = Result<T, AppError>;
