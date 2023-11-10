use thiserror::Error;

#[derive(Error, Debug)]
pub enum CzasError {
    #[error("Failed to parse datetime")]
    Error,
}
