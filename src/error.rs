use thiserror::Error;

#[derive(Error, Debug)]
pub enum FatalError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] rusqlite::Error),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Word analyzer initialization failed: {0}")]
    WordAnalyzer(String),

    #[error("Failed to write to standard output: {0}")]
    WriteError(#[from] std::io::Error),
}

#[derive(Error, Debug)]
pub enum RecoverableError {
    #[error("Invalid input format: expected 'gyngy' format, got '{0}'")]
    InvalidInputFormat(String),

    #[error("No words match current constraints")]
    NoMatchingWords,
    #[error("Network timeout occurred")]
    NetworkTimeout,
}
