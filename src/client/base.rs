use crate::{
    error::ApiClientError,
    models::{ApiResponse, ApiResponseResult},
};
use reqwest::{header::HeaderMap, Client, StatusCode};
use serde::Deserialize;

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

    pub async fn get<T: for<'de> Deserialize<'de>>(
        &self,
        base_url: &str,
        endpoint: &str,
        params: &[(&str, &str)],
    ) -> Result<T, ApiClientError> {
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

        let response = request.send().await?;

        match response.status() {
            StatusCode::OK => {
                let response_body = response.json::<ApiResponse<T>>().await.map_err(|e| {
                    if e.is_decode() {
                        ApiClientError::JsonStructureError(format!(
                            "Failed to decode JSON structure: {}",
                            e
                        ))
                    } else {
                        ApiClientError::NetworkError(e)
                    }
                })?;

                if response_body.ok {
                    if let ApiResponseResult::Success { result } = response_body.data {
                        Ok(result)
                    } else {
                        Err(ApiClientError::UnexpectedResponse)
                    }
                } else {
                    if let ApiResponseResult::Error { error, code } = response_body.data {
                        Err(ApiClientError::ApiError(format!(
                            "Error {}: {}",
                            code, error
                        )))
                    } else {
                        Err(ApiClientError::UnexpectedResponse)
                    }
                }
            }
            StatusCode::UNPROCESSABLE_ENTITY => {
                let error_text = response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Unprocessable Entity".to_string());
                Err(ApiClientError::ValidationError(error_text))
            }
            StatusCode::GATEWAY_TIMEOUT => {
                let error_text = response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Gateway Timeout".to_string());
                Err(ApiClientError::LiteServerTimeout(error_text))
            }
            _ => {
                let error_text = response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Unknown error".to_string());
                Err(ApiClientError::ApiError(error_text))
            }
        }
    }
}
