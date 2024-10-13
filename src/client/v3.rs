#![allow(dead_code)]
#![allow(unused_variables)]
use super::base::Network;
use crate::client::base::{ApiKey, BaseApiClient};

pub struct ApiClientV3 {
    base_client: BaseApiClient,
    base_url: String,
}

impl ApiClientV3 {
    pub fn new(network: Network, api_key: Option<ApiKey>) -> Self {
        /*
        let base_url = match network {
            Network::Mainnet => "https://toncenter.com/api/v3/",
            Network::Testnet => "https://testnet.toncenter.com/api/v3/",
        }.to_string();

        Self {
            base_client: BaseApiClient::new(api_key),
            base_url,
        }
        */

        unimplemented!("V3 API implementation will be provided in the future.");
    }
}
