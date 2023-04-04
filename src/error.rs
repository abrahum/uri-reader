use hyper::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UReadError {
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    Uri(#[from] hyper::http::uri::InvalidUri),
    #[error("{0}")]
    Base64(#[from] base64::DecodeError),
    #[error("{0}")]
    Hyper(#[from] hyper::Error),
    #[error("{0}")]
    HttpStatus(StatusCode),

    #[error("Unsupported scheme: {0}")]
    UnsupportedScheme(String),
    #[error("Empty scheme")]
    EmptyScheme,
    #[error("Https disabled")]
    HttpsDisabled
}
