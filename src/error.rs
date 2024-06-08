use reqwest::header::InvalidHeaderValue;
use reqwest::Error as ReqwestError;
use serde_json::Error as SerdeError;
use std::error::Error;
use std::fmt;
use url::ParseError as UrlParseError;

#[derive(Debug)]
pub enum ToncenterError {
    InvalidInput(InvalidInput),
    ProcessingError(ProcessingError),
    RateLimitExceeded,
    HttpClientError { code: u32, message: String },
    HttpServerError { code: u32, message: String },
}

#[derive(Debug)]
pub enum InvalidInput {
    HeaderValue(InvalidHeaderValue),
    UrlParse(UrlParseError),
}

#[derive(Debug)]
pub enum ProcessingError {
    Network(ReqwestError),
    Deserialization(SerdeError),
}

impl fmt::Display for ToncenterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ToncenterError::InvalidInput(err) => write!(f, "Invalid input: {}", err),
            ToncenterError::ProcessingError(err) => write!(f, "Processing error: {}", err),
            ToncenterError::RateLimitExceeded => write!(f, "Rate limit exceeded"),
            ToncenterError::HttpClientError { code, message } => {
                write!(f, "Client error {}: {}", code, message)
            }
            ToncenterError::HttpServerError { code, message } => {
                write!(f, "Server error {}: {}", code, message)
            }
        }
    }
}

impl fmt::Display for InvalidInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InvalidInput::HeaderValue(err) => write!(f, "Invalid header value: {}", err),
            InvalidInput::UrlParse(err) => write!(f, "URL parse error: {}", err),
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

impl Error for ToncenterError {}

impl From<InvalidHeaderValue> for ToncenterError {
    fn from(err: InvalidHeaderValue) -> ToncenterError {
        ToncenterError::InvalidInput(InvalidInput::HeaderValue(err))
    }
}

impl From<UrlParseError> for ToncenterError {
    fn from(err: UrlParseError) -> ToncenterError {
        ToncenterError::InvalidInput(InvalidInput::UrlParse(err))
    }
}

impl From<ReqwestError> for ToncenterError {
    fn from(err: ReqwestError) -> ToncenterError {
        ToncenterError::ProcessingError(ProcessingError::Network(err))
    }
}

impl From<SerdeError> for ToncenterError {
    fn from(err: SerdeError) -> ToncenterError {
        ToncenterError::ProcessingError(ProcessingError::Deserialization(err))
    }
}
