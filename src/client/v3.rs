#![allow(dead_code)]
#![allow(unused_variables)]
use crate::client::base::{ApiKey, Network};
use crate::client::base_v3::BaseApiClientV3;
use crate::error::ToncenterError;
use crate::models_v3::{MessageSuccessResponse, RawFullAccountStateV3};

pub struct ApiClientV3 {
    base_client: BaseApiClientV3,
    base_url: String,
}

impl ApiClientV3 {
    pub fn new(network: Network, api_key: Option<ApiKey>) -> Self {
        let base_url = match network {
            Network::Mainnet => "https://toncenter.com/api/v3/",
            Network::Testnet => "https://testnet.toncenter.com/api/v3/",
        }
        .to_string();

        Self {
            base_client: BaseApiClientV3::new(api_key),
            base_url,
        }
    }

    /// Get basic information about the address: balance, code, data, last_transaction_id.
    ///
    /// # Parameters
    ///
    /// * `address` - Identifier of the target TON account in any form.
    pub async fn get_address_information(
        &self,
        address: &str,
    ) -> Result<RawFullAccountStateV3, ToncenterError> {
        let params = [("address", address), ("use_v2", "true")];

        self.base_client
            .get(&self.base_url, "addressInformation", &params)
            .await
    }

    /// Send an external message to the TON network
    ///
    /// # Parameters
    ///
    /// * `boc` - Message in boc base64 format
    pub async fn send_message(&self, boc: &str) -> Result<MessageSuccessResponse, ToncenterError> {
        let body = serde_json::json!({
            "boc": boc,
        });
        self.base_client
            .post_api(&self.base_url, "message", &body)
            .await
    }
}
