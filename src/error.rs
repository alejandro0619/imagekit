use thiserror;

use std::fmt::Debug;

/// Response error should represent every API error code.
/// More information at: https://docs.imagekit.io/api-reference/api-introduction#error-codes
#[derive(thiserror::Error, Debug)]
pub enum ResponseError {
    #[error("Bad request. Reason: {0}")]
    BadRequest(String),
    #[error("Unauthorized. Reason: {0}")]
    Unauthorized(String),
    #[error("Forbidden request. Reason: {0}")]
    Forbidden(String),
    #[error("Too many requests, throttle down the number of requests to stay within the rate limit")]
    TooManyRequests,
    #[error("Internal server error")]
    ServerError
}

/// Error represents every possible failure from the incorrect usage of either the API or the wrapper
#[derive(thiserror::Error, Debug)]
pub enum Error {
  /// API Errors
  #[error("API Error. \n{0}")]
  API(ResponseError),
  /// Reqwest Errors, such as failure when deserializing the response, making the request, setting auth headers or initializing the client, etc.
  #[error("Reqwest Error. \n{0}")]
  Reqwest(#[from] reqwest::Error),
  #[error("A problem ocurred when resolving the enviroment variables: {0}")]
  Env(#[from] std::env::VarError)
} 