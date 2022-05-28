use thiserror::Error;

#[derive(Debug, Error)]
pub enum GeniusError {
    #[error("{0}")]
    InternalServerError(String),
    #[error("Parse error: {0}")]
    ParseError(String),
    #[error("Request error: {0}")]
    RequestError(String),
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
    #[error("Not found: {0}")]
    NotFound(String),
}
