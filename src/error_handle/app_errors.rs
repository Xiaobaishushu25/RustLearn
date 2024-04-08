use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("anyhow error:`{0}`")]
    AnyHow(#[from] anyhow::Error),
    #[error("io::Error:`{0}`")]
    IoError(#[from] io::Error),
    #[error("serde_json::Error:`{0}`")]
    SerdeError(#[from] serde_json::Error),
    #[error("Not enough data is available to parse a message")]
    IncompleteError,
    #[error("this is a error:`{0}`")]
    ErrorDescribe(String),
    //其实这个错误类型大概和SerdeError差不多，但是这个错误是协议错误，没有按照约定的格式发送消息，无法解析，应该直接断开连接。
    #[error("cant parse message!")]
    MessageFormatError,
    // #[error("http::ParseError:`{0}`")]
    // ParseError(#[from] ParseError),
    // #[error("sea_orm::DbErr:Error:`{0}`")]
    // DbErr(#[from] sea_orm::DbErr),
}
pub type AppResult<T> = Result<T, AppError>;
