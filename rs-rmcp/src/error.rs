//! 错误处理

#[allow(unused)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    SerdeJson(#[from] serde_json::error::Error),

    #[error(transparent)]
    Regex(#[from] regex::Error),

    #[error(transparent)]
    PathExtractionError(#[from] PathExtractionError),
}

/// Path 自定义错误类型
#[allow(unused)]
#[derive(Debug, thiserror::Error)]
pub enum PathExtractionError {
    #[error("Missing URL parameters")]
    MissingParams,
    #[error("Invalid URL parameter format")]
    InvalidFormat,
    #[error("Unsupported target type")]
    UnsupportedType,
}
