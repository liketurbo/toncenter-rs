use crate::{
    error::ToncenterError,
    models::{ApiResponse, ApiResponseResult},
};
use log::debug;
use reqwest::{header::HeaderMap, Client};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum Network {
    Mainnet,
    Testnet,
}

#[derive(Debug)]
pub enum ApiKey {
    Header(String),
    Query(String),
}

#[derive(Debug)]
pub struct BaseApiClient {
    client: Client,
    api_key: Option<ApiKey>,
}

impl BaseApiClient {
    pub fn new(api_key: Option<ApiKey>) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }

    pub async fn get<T: for<'de> Deserialize<'de> + std::fmt::Debug>(
        &self,
        base_url: &str,
        endpoint: &str,
        params: &[(&str, &str)],
    ) -> Result<T, ToncenterError> {
        let mut headers = HeaderMap::new();
        let mut query_params = params.to_vec();

        if let Some(ref key) = self.api_key {
            match key {
                ApiKey::Header(key) => {
                    headers.insert("x-api-key", key.parse()?);
                }
                ApiKey::Query(key) => {
                    query_params.push(("api_key", key));
                }
            };
        }

        let url = format!("{}{}", base_url, endpoint);
        let url_with_params = reqwest::Url::parse_with_params(&url, query_params)?;
        let request = self.client.get(url_with_params).headers(headers);
        debug!("Request after processing: {:?}", request);

        let response = request.send().await?;
        debug!("Received response: {:?}", response);

        let response_text = response.text().await?;
        debug!("Response text: {}", response_text);

        let response_body: ApiResponse<T> = serde_json::from_str(&response_text)?;
        debug!("Response body: {:?}", response_body);

        if response_body.ok {
            if let ApiResponseResult::Success { result } = response_body.data {
                return Ok(result);
            }

            return Err(ToncenterError::HttpServerError {
                code: 500,
                message: "Invalid response from server, expected 'result'".to_string(),
            });
        } else {
            if let ApiResponseResult::Error {
                result,
                error,
                code,
            } = response_body.data
            {
                let error_message = error
                    .or(result)
                    .unwrap_or_else(|| "Unknown error".to_string());
                if code == 429 {
                    return Err(ToncenterError::RateLimitExceeded);
                } else if (400..500).contains(&code) {
                    return Err(ToncenterError::HttpClientError {
                        code,
                        message: error_message,
                    });
                } else {
                    return Err(ToncenterError::HttpServerError {
                        code,
                        message: error_message,
                    });
                }
            }

            return Err(ToncenterError::HttpServerError {
                code: 500,
                message: "Invalid response from server, expected 'result' or 'error'".to_string(),
            });
        }
    }

    pub async fn post<T: for<'de> Deserialize<'de> + std::fmt::Debug, B: Serialize>(
        &self,
        base_url: &str,
        endpoint: &str,
        body: &B,
    ) -> Result<T, ToncenterError> {
        let mut headers = HeaderMap::new();
        let mut query_params = vec![];

        if let Some(ref key) = self.api_key {
            match key {
                ApiKey::Header(key) => {
                    headers.insert("x-api-key", key.parse()?);
                }
                ApiKey::Query(key) => {
                    query_params.push(("api_key", key));
                }
            };
        }

        let url = format!("{}{}", base_url, endpoint);
        let url_with_params = reqwest::Url::parse_with_params(&url, query_params)?;
        let request = self
            .client
            .post(url_with_params)
            .headers(headers)
            .json(body);
        debug!("Request after processing: {:?}", request);

        let response = request.send().await?;
        debug!("Received response: {:?}", response);

        let response_text = response.text().await?;
        debug!("Response text: {}", response_text);

        let response_body: ApiResponse<T> = serde_json::from_str(&response_text)?;
        debug!("Response body: {:?}", response_body);

        if response_body.ok {
            if let ApiResponseResult::Success { result } = response_body.data {
                return Ok(result);
            }

            return Err(ToncenterError::HttpServerError {
                code: 500,
                message: "Invalid response from server, expected 'result'".to_string(),
            });
        } else {
            if let ApiResponseResult::Error {
                result,
                error,
                code,
            } = response_body.data
            {
                let error_message = error
                    .or(result)
                    .unwrap_or_else(|| "Unknown error".to_string());
                if code == 429 {
                    return Err(ToncenterError::RateLimitExceeded);
                } else if (400..500).contains(&code) {
                    return Err(ToncenterError::HttpClientError {
                        code,
                        message: error_message,
                    });
                } else {
                    return Err(ToncenterError::HttpServerError {
                        code,
                        message: error_message,
                    });
                }
            }

            return Err(ToncenterError::HttpServerError {
                code: 500,
                message: "Invalid response from server, expected 'result' or 'error'".to_string(),
            });
        }
    }
}
