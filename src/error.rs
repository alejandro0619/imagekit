use thiserror;

use std::fmt::Debug;

/// Response error should represent every API error code.
/// More information at: https://docs.imagekit.io/api-reference/api-introduction#error-codes
#[derive(thiserror::Error, Debug)]
pub enum ApiError {
    #[error("Bad request. Reason: {0}")]
    BadRequest(String),
    #[error("Unauthorized. Reason: {0}")]
    Unauthorized(String),
    #[error("Forbidden request. Reason: {0}")]
    Forbidden(String),
    #[error("Too many requests, throttle down the number of requests to stay within the rate limit")]
    TooManyRequests,
    #[error("Internal server error")]
    ServerError, 
}

/// Error represents every possible failure from the incorrect usage of either the API or the wrapper
#[derive(thiserror::Error, Debug)]
pub enum Error {
  /// API Errors
  #[error("API Error. \n{0}")]
  API(ApiError),
  /// Reqwest Errors, such as failure when deserializing the response, making the request, setting auth headers or initializing the client, etc.
  #[error("Reqwest Error. \n{0}")]
  HttpClientError(#[from] reqwest::Error),
  /// Enviroment variables errors, such as non-existing env variables.
  #[error("A problem has ocurred when resolving the enviroment variables: {0}")]
  EnvError(#[from] std::env::VarError),

  /// Represents any error relate to the use of serde to deserialize JSON into response structs
  #[error("A problem has ocurred while deserializing with the : {0}, and the code: {1}")]
  ParsingError(String, u16)

} 