use reqwest::header::InvalidHeaderValue;
use reqwest::Error as ReqwestError;
use serde_json::Error as SerdeError;
use std::error::Error;
use std::fmt;
use url::ParseError as UrlParseError;

#[derive(Debug)]
pub enum ApiError {
    InvalidUserInput(InvalidUserInputError),
    ProcessingError(ProcessingError),
    RateLimitExceeded,
    ClientError { code: u32, message: String },
    ServerError { code: u32, message: String },
}

#[derive(Debug)]
pub enum InvalidUserInputError {
    InvalidHeaderValue(InvalidHeaderValue),
    UrlParseError(UrlParseError),
}

#[derive(Debug)]
pub enum ProcessingError {
    Network(ReqwestError),
    Deserialization(SerdeError),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::InvalidUserInput(err) => write!(f, "Invalid user input: {}", err),
            ApiError::ProcessingError(err) => write!(f, "Processing error: {}", err),
            ApiError::RateLimitExceeded => write!(f, "Rate limit exceeded"),
            ApiError::ClientError { code, message } => {
                write!(f, "Client error {}: {}", code, message)
            }
            ApiError::ServerError { code, message } => {
                write!(f, "Server error {}: {}", code, message)
            }
        }
    }
}

impl fmt::Display for InvalidUserInputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InvalidUserInputError::InvalidHeaderValue(err) => {
                write!(f, "Invalid header value: {}", err)
            }
            InvalidUserInputError::UrlParseError(err) => write!(f, "URL parse error: {}", err),
        }
    }
}

impl fmt::Display for ProcessingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProcessingError::Network(err) => write!(f, "Network error: {}", err),
            ProcessingError::Deserialization(err) => write!(f, "Deserialization error: {}", err),
        }
    }
}

impl Error for ApiError {}

impl From<InvalidHeaderValue> for ApiError {
    fn from(err: InvalidHeaderValue) -> ApiError {
        ApiError::InvalidUserInput(InvalidUserInputError::InvalidHeaderValue(err))
    }
}

impl From<UrlParseError> for ApiError {
    fn from(err: UrlParseError) -> ApiError {
        ApiError::InvalidUserInput(InvalidUserInputError::UrlParseError(err))
    }
}

impl From<ReqwestError> for ApiError {
    fn from(err: ReqwestError) -> ApiError {
        ApiError::ProcessingError(ProcessingError::Network(err))
    }
}

impl From<SerdeError> for ApiError {
    fn from(err: SerdeError) -> ApiError {
        ApiError::ProcessingError(ProcessingError::Deserialization(err))
    }
}
