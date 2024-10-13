use crate::{client::base::ApiKey, error::ToncenterError, models_v3::ApiResponseResultV3};
use log::debug;
use reqwest::{header::HeaderMap, Client};
use serde::{de::DeserializeOwned, Serialize};

#[derive(Debug)]
pub struct BaseApiClientV3 {
    client: Client,
    api_key: Option<ApiKey>,
}

impl BaseApiClientV3 {
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

        let code = response.status().as_u16() as u32;
        debug!("Received response: {:?}", response);

        let response_text = response.text().await?;
        debug!("Response text: {}", response_text);

        let response_result: ApiResponseResultV3<T> = serde_json::from_str(&response_text)?;

        match response_result {
            ApiResponseResultV3::Success(t) => Ok(t),
            ApiResponseResultV3::Error { error } => {
                let message = error.unwrap_or_else(|| "Unknown error".to_string());

                Err(ToncenterError::HttpServerError { code, message })
            }
        }
    }

    pub async fn get<T: DeserializeOwned + std::fmt::Debug>(
        &self,
        base_url: &str,
        endpoint: &str,
        params: &[(&str, &str)],
    ) -> Result<T, ToncenterError> {
        // let response_body: ApiResponseResultV3<T> =
        self.send_request(
            reqwest::Method::GET,
            base_url,
            endpoint,
            params,
            None::<&serde_json::Value>,
        )
        .await
    }

    #[allow(dead_code)]
    pub async fn post_api<T: DeserializeOwned + std::fmt::Debug>(
        &self,
        base_url: &str,
        endpoint: &str,
        body: &impl Serialize,
    ) -> Result<T, ToncenterError> {
        // let response_body: ApiResponseResultV3<T> =
        self.send_request(reqwest::Method::POST, base_url, endpoint, &[], Some(body))
            .await
    }
}
