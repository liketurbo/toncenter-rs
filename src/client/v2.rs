use crate::client::base::{ApiKey, BaseApiClient};
use crate::error::ApiClientError;
use crate::models::{AddressInformation, ExtendedAddressInformation, WalletInformation};

use super::base::Network;

pub struct ApiClientV2 {
    base_client: BaseApiClient,
    base_url: String,
}

impl ApiClientV2 {
    pub fn new(network: Network, api_key: Option<ApiKey>) -> Self {
        let base_url = match network {
            Network::Mainnet => "https://toncenter.com/api/v2/",
            Network::Testnet => "https://testnet.toncenter.com/api/v2/",
        }
        .to_string();
        Self {
            base_client: BaseApiClient::new(api_key),
            base_url,
        }
    }

    /// Get basic information about the address: balance, code, data, last_transaction_id.
    ///
    /// # Parameters
    ///
    /// * `address` - A string representing the identifier of the target TON account in any form.
    pub async fn get_address_information(
        &self,
        address: &str,
    ) -> Result<AddressInformation, ApiClientError> {
        self.base_client
            .get(
                &self.base_url,
                "getAddressInformation",
                &[("address", address)],
            )
            .await
    }

    /// Get extended information about the address.
    ///
    /// Similar to the previous method but tries to parse additional information for known contract types.
    /// This method is based on tonlib's function `getAccountState`.
    /// For detecting wallets, we recommend using `getWalletInformation`.
    ///
    /// # Parameters
    ///
    /// * `address` - A string representing the identifier of the target TON account in any form.
    pub async fn get_extended_address_information(
        &self,
        address: &str,
    ) -> Result<ExtendedAddressInformation, ApiClientError> {
        self.base_client
            .get(
                &self.base_url,
                "getExtendedAddressInformation",
                &[("address", address)],
            )
            .await
    }

    /// Retrieve wallet information.
    ///
    /// This method parses contract state and currently supports more wallet types than getExtendedAddressInformation:
    /// simple wallet, standard wallet, v3 wallet, v4 wallet.
    ///
    /// # Parameters
    ///
    /// * `address` - A string representing the identifier of the target TON account in any form.
    pub async fn get_wallet_information(
        &self,
        address: &str,
    ) -> Result<WalletInformation, ApiClientError> {
        self.base_client
            .get(
                &self.base_url,
                "getWalletInformation",
                &[("address", address)],
            )
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_address_information() {
        let api_client = ApiClientV2::new(Network::Testnet, None);
        let result = api_client
            .get_address_information("0QCbOix87iy37AwRCWaYhJHzc2gXE_WnAG5vVEAySNT7zClz")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_extended_address_information() {
        let api_client = ApiClientV2::new(Network::Testnet, None);
        let result = api_client
            .get_extended_address_information("0QCbOix87iy37AwRCWaYhJHzc2gXE_WnAG5vVEAySNT7zClz")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_wallet_information() {
        let api_client = ApiClientV2::new(Network::Testnet, None);
        let result = api_client
            .get_wallet_information("0QCbOix87iy37AwRCWaYhJHzc2gXE_WnAG5vVEAySNT7zClz")
            .await;
        assert!(result.is_ok());
    }
}
