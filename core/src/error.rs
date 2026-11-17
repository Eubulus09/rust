use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("could not read or write the data file: {0}")]
    Json(#[from] serde_json::Error),

    #[error("{0}")]
    Validation(String),

    #[error("task {0} not found")]
    NotFound(i64),
}
