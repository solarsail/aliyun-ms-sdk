use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("failed to send request: {source}")]
    SendError {
        #[from]
        source: reqwest::Error,
    },
    #[error("api error: [{code}] request_id: {request_id}, {message}")]
    ApiError {
        code: String,
        request_id: String,
        message: String,
    },
    #[error("response is initiated by a different request type")]
    IncorrectTypeError,
}
