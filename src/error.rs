use thiserror::Error;

#[derive(Debug, Error)]
#[error("response is initiated by a different request type")]
pub struct IncorrectTypeError;

#[derive(Debug, Error)]
pub enum RequestError {
    #[error("failed to send request: {source}")]
    SendError {
        #[from]
        source: reqwest::Error,
    },
    #[error("api error: [{code}] {message}")]
    ApiError {
        code: u16,
        message: String,
    }
}