use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AddressInformation {
    #[serde(rename = "@type")]
    pub type_field: String,
    pub balance: String,
    pub code: Option<String>,
    pub data: Option<String>,
    pub last_transaction_id: TransactionId,
    pub block_id: BlockId,
    pub frozen_hash: Option<String>,
    pub sync_utime: u64,
    #[serde(rename = "@extra")]
    pub extra: String,
    pub state: String,
}

#[derive(Debug, Deserialize)]
pub struct ExtendedAddressInformation {
    #[serde(rename = "@type")]
    pub type_field: String,
    pub address: AccountAddress,
    pub balance: String,
    pub last_transaction_id: TransactionId,
    pub block_id: BlockId,
    pub sync_utime: u64,
    pub account_state: AccountState,
    pub revision: i32,
    #[serde(rename = "@extra")]
    pub extra: String,
}

#[derive(Debug, Deserialize)]
pub struct AccountAddress {
    #[serde(rename = "@type")]
    pub type_field: String,
    pub account_address: String,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "@type")]
pub enum AccountState {
    #[serde(rename = "uninited.accountState")]
    Uninited { frozen_hash: String },
    #[serde(rename = "wallet.v4.accountState")]
    WalletV4 { wallet_id: String, seqno: u32 },
}

#[derive(Debug, Deserialize)]
pub struct TransactionId {
    #[serde(rename = "@type")]
    pub type_field: String,
    pub lt: String,
    pub hash: String,
}

#[derive(Debug, Deserialize)]
pub struct BlockId {
    #[serde(rename = "@type")]
    pub type_field: String,
    pub workchain: i32,
    pub shard: String,
    pub seqno: u32,
    pub root_hash: String,
    pub file_hash: String,
}

#[derive(Debug, Deserialize)]
pub struct WalletInformation {
    pub wallet: bool,
    pub balance: String,
    pub account_state: String,
    pub wallet_type: String,
    pub seqno: u32,
    pub last_transaction_id: TransactionId,
    pub wallet_id: u64,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ApiResponseResult<T> {
    Success { result: T },
    Error { error: String, code: i32 },
}

#[derive(Debug, Deserialize)]
pub struct ApiResponse<T> {
    pub ok: bool,
    #[serde(flatten)]
    pub data: ApiResponseResult<T>,
}
