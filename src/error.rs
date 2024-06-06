use reqwest::header::InvalidHeaderValue;
use reqwest::Error as ReqwestError;
use serde_json::Error as SerdeError;
use std::error::Error;
use std::fmt;
use url::ParseError as UrlParseError;

#[derive(Debug)]
pub enum ApiClientError {
    NetworkError(ReqwestError),
    DeserializationError(SerdeError),
    ApiError(String),
    UrlParseError(UrlParseError),
    InvalidHeaderValue(InvalidHeaderValue),
    UnexpectedResponse,
    ValidationError(String),
    LiteServerTimeout(String),
    JsonStructureError(String),
}

impl fmt::Display for ApiClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiClientError::NetworkError(err) => write!(f, "Network error: {}", err),
            ApiClientError::DeserializationError(err) => {
                write!(f, "Deserialization error: {}", err)
            }
            ApiClientError::ApiError(err) => write!(f, "API error: {}", err),
            ApiClientError::UrlParseError(err) => write!(f, "URL parse error: {}", err),
            ApiClientError::InvalidHeaderValue(err) => write!(f, "Invalid header value: {}", err),
            ApiClientError::UnexpectedResponse => write!(f, "Unexpected response structure"),
            ApiClientError::ValidationError(err) => write!(f, "Validation error: {}", err),
            ApiClientError::LiteServerTimeout(err) => write!(f, "Lite Server Timeout: {}", err),
            ApiClientError::JsonStructureError(err) => write!(f, "JSON structure error: {}", err),
        }
    }
}

impl Error for ApiClientError {}

impl From<ReqwestError> for ApiClientError {
    fn from(err: ReqwestError) -> ApiClientError {
        ApiClientError::NetworkError(err)
    }
}

impl From<SerdeError> for ApiClientError {
    fn from(err: SerdeError) -> ApiClientError {
        ApiClientError::DeserializationError(err)
    }
}

impl From<UrlParseError> for ApiClientError {
    fn from(err: UrlParseError) -> ApiClientError {
        ApiClientError::UrlParseError(err)
    }
}

impl From<InvalidHeaderValue> for ApiClientError {
    fn from(err: InvalidHeaderValue) -> ApiClientError {
        ApiClientError::InvalidHeaderValue(err)
    }
}
