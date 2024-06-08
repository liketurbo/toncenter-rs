use crate::{
    error::ToncenterError,
    models::{ApiResponse, ApiResponseResult, JsonRpcResponse, JsonRpcResult},
};
use log::debug;
use reqwest::{header::HeaderMap, Client};
use serde::{de::DeserializeOwned, Serialize};

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

    async fn send_request<T: DeserializeOwned + std::fmt::Debug>(
        &self,
        method: reqwest::Method,
        base_url: &str,
        endpoint: &str,
        params: &[(&str, &str)],
        body: Option<&impl Serialize>,
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
        let request_builder = match method {
            reqwest::Method::GET => self.client.get(url_with_params).headers(headers),
            reqwest::Method::POST => {
                let builder = self.client.post(url_with_params).headers(headers);
                if let Some(body) = body {
                    builder.json(body)
                } else {
                    builder
                }
            }
            _ => unimplemented!(),
        };

        debug!("Request after processing: {:?}", request_builder);

        let response = request_builder.send().await?;
        debug!("Received response: {:?}", response);

        let response_text = response.text().await?;
        debug!("Response text: {}", response_text);

        let response_body: T = serde_json::from_str(&response_text)?;
        debug!("Response body: {:?}", response_body);

        Ok(response_body)
    }

    pub async fn get<T: DeserializeOwned + std::fmt::Debug>(
        &self,
        base_url: &str,
        endpoint: &str,
        params: &[(&str, &str)],
    ) -> Result<T, ToncenterError> {
        let response_body: ApiResponse<T> = self
            .send_request(
                reqwest::Method::GET,
                base_url,
                endpoint,
                params,
                None::<&serde_json::Value>,
            )
            .await?;
        self.handle_api_response(response_body).await
    }

    pub async fn post_api<T: DeserializeOwned + std::fmt::Debug>(
        &self,
        base_url: &str,
        endpoint: &str,
        body: &impl Serialize,
    ) -> Result<T, ToncenterError> {
        let response_body: ApiResponse<T> = self
            .send_request(reqwest::Method::POST, base_url, endpoint, &[], Some(body))
            .await?;
        self.handle_api_response(response_body).await
    }

    pub async fn post_rpc<T: DeserializeOwned + std::fmt::Debug>(
        &self,
        base_url: &str,
        endpoint: &str,
        body: &impl Serialize,
    ) -> Result<T, ToncenterError> {
        let response_body: JsonRpcResponse<T> = self
            .send_request(reqwest::Method::POST, base_url, endpoint, &[], Some(body))
            .await?;

        if response_body.ok {
            if let JsonRpcResult::Success { result } = response_body.data {
                return Ok(result);
            }

            unreachable!("Invalid response from server, expected 'result'");
        }

        if let JsonRpcResult::Error {
            result,
            error,
            code,
        } = response_body.data
        {
            let error_message = error
                .or(result)
                .unwrap_or_else(|| "Unknown error".to_string());
            self.handle_error(code, error_message)?;
        }

        unreachable!("Invalid response from server, expected 'result' or 'error'");
    }

    async fn handle_api_response<T: DeserializeOwned + std::fmt::Debug>(
        &self,
        response_body: ApiResponse<T>,
    ) -> Result<T, ToncenterError> {
        if response_body.ok {
            if let ApiResponseResult::Success { result } = response_body.data {
                return Ok(result);
            }

            unreachable!("Invalid response from server, expected 'result'");
        }

        if let ApiResponseResult::Error {
            result,
            error,
            code,
        } = response_body.data
        {
            let error_message = error
                .or(result)
                .unwrap_or_else(|| "Unknown error".to_string());
            self.handle_error(code, error_message)?;
        }

        unreachable!("Invalid response from server, expected 'result' or 'error'");
    }

    fn handle_error(&self, code: u32, message: String) -> Result<(), ToncenterError> {
        if code == 429 {
            Err(ToncenterError::RateLimitExceeded)
        } else if (400..500).contains(&code) {
            Err(ToncenterError::HttpClientError { code, message })
        } else {
            Err(ToncenterError::HttpServerError { code, message })
        }
    }
}
