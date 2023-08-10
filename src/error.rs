use anyhow;
use reqwest::{header, StatusCode};
use std::env::VarError;
use thiserror;

/// Custom error handling
/// Check this out: https://docs.imagekit.io/api-reference/api-introduction#error-codes
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// Any http error that raises
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    /// Invalid parsing of the headers
    #[error("Invalid Header: {0}")]
    Header(#[from] header::InvalidHeaderValue),
    // Env doesn't exist or it's not parseable
    #[error("Couldn't find or parse: {0}")]
    Env(#[from] VarError),
    // Error while deserializing the response
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    /// Unauthorized 401 error code
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
    /// Forbidden 403 error code
    #[error("Forbidden: {0}")]
    Forbidden(String),
    /// Too many requests 429 error code
    #[error("Too many requests: {0}")]
    TooManyRequests(String),
    /// Internal server error 500, 502, 503, 504 error code
    #[error("Internal server error: {0}")]
    InternalServerError(String),
    /// Bad Request error 400 error code
    #[error("Bad Request: {0}")]
    BadRequest(String),
    /// This error raises when no file were found by the given searching-criteria
    #[error("Search error occurred: {0}")]
    SearchError(String),
    // Represent any error that is not covered in the above cases nor in the API documentation. Give it a cool name, not just Unknown
    #[error("Unknown error: {0}")]
    UnknownAPIError(String),

}

impl Error {
    /// This function will parse a statusCode into an Error variant
    pub fn from_error_code(code: StatusCode, reason: &str) -> Self {
        match code {
            StatusCode::UNAUTHORIZED => Self::Forbidden(reason.to_string()),
            StatusCode::FORBIDDEN => Self::Forbidden(reason.to_string()),
            StatusCode::TOO_MANY_REQUESTS => Self::TooManyRequests(reason.to_string()),
            StatusCode::BAD_GATEWAY
            | StatusCode::INTERNAL_SERVER_ERROR
            | StatusCode::SERVICE_UNAVAILABLE
            | StatusCode::GATEWAY_TIMEOUT => Self::InternalServerError(reason.to_string()),
            StatusCode::BAD_REQUEST => Self::BadRequest(reason.to_string()),
            // Represent any other error from the API that is not covered in the above cases nor in the documentation
            _ => Self::UnknownAPIError(reason.to_string()),
        }
    }
}

pub type Result<T> = anyhow::Result<T, Error>;
