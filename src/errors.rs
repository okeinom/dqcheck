use thiserror::Error;

#[derive(Error, Debug)]/// Define a custom error type for the application

pub enum DqError{
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("CSV error: {0}")]
    Csv(#[from] csv::Error),

    #[error("JSON pase error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Rules error: {0}")]
    Rules(String),
}