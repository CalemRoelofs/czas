use thiserror::Error;

#[derive(Error, Debug)]
pub enum CzasError {
    #[error("Failed to parse datetime")]
    Error,
    #[error("Timestamp is invalid")]
    ChronoError(#[from] chrono::ParseError),
}
