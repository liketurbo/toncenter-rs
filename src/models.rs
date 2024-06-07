use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ApiResponse<T> {
    pub ok: bool,
    #[serde(flatten)]
    pub data: ApiResponseResult<T>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ApiResponseResult<T> {
    Success {
        result: T,
    },
    Error {
        result: Option<String>,
        error: Option<String>,
        code: u32,
    },
}

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
pub struct Transaction {
    #[serde(rename = "@type")]
    pub type_field: String,
    pub address: AccountAddress,
    pub utime: u64,
    pub data: String,
    pub transaction_id: TransactionId,
    pub fee: String,
    pub storage_fee: String,
    pub other_fee: String,
    pub in_msg: Option<Message>,
    pub out_msgs: Vec<Message>,
}

#[derive(Debug, Deserialize)]
pub struct Message {
    #[serde(rename = "@type")]
    pub type_field: String,
    pub source: Option<String>,
    pub destination: String,
    pub value: String,
    pub fwd_fee: String,
    pub ihr_fee: String,
    pub created_lt: String,
    pub body_hash: String,
    pub msg_data: MsgData,
    pub message: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct MsgData {
    #[serde(rename = "@type")]
    pub type_field: String,
    pub body: Option<String>,
    pub init_state: Option<String>,
    pub text: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TokenData {
    pub init: bool,
    pub index: u32,
    pub owner_address: String,
    pub collection_address: String,
    pub content: TokenContent,
    pub contract_type: String,
}

#[derive(Debug, Deserialize)]
pub struct TokenContent {
    pub r#type: String,
    pub data: String,
}

#[derive(Debug, Deserialize)]
pub struct DetectAddressResult {
    pub raw_form: String,
    pub bounceable: AddressFormat,
    pub non_bounceable: AddressFormat,
    pub given_type: String,
    pub test_only: bool,
}

#[derive(Debug, Deserialize)]
pub struct AddressFormat {
    pub b64: String,
    pub b64url: String,
}

#[derive(Debug, Deserialize)]
pub struct MasterchainInfo {
    #[serde(rename = "@type")]
    pub type_field: String,
    pub last: BlockIdExt,
    pub state_root_hash: String,
    pub init: BlockIdExt,
    #[serde(rename = "@extra")]
    pub extra: String,
}

#[derive(Debug, Deserialize)]
pub struct BlockIdExt {
    #[serde(rename = "@type")]
    pub type_field: String,
    pub workchain: i32,
    pub shard: String,
    pub seqno: u32,
    pub root_hash: String,
    pub file_hash: String,
    #[serde(rename = "@extra")]
    pub extra: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct MasterchainBlockSignatures {
    #[serde(rename = "@type")]
    pub type_field: String,
    pub id: BlockIdExt,
    pub signatures: Vec<BlockSignature>,
    #[serde(rename = "@extra")]
    pub extra: String,
}

#[derive(Debug, Deserialize)]
pub struct BlockSignature {
    #[serde(rename = "@type")]
    pub type_field: String,
    pub node_id_short: String,
    pub signature: String,
}

#[derive(Debug, Deserialize)]
pub struct ShardBlockProof {
    #[serde(rename = "@type")]
    pub type_field: String,
    pub from: BlockIdExt,
    pub mc_id: BlockIdExt,
    pub links: Vec<ShardBlockLink>,
    pub mc_proof: Vec<ShardBlockProofEntry>,
    #[serde(rename = "@extra")]
    pub extra: String,
}

#[derive(Debug, Deserialize)]
pub struct ShardBlockLink {
    #[serde(rename = "@type")]
    pub type_field: String,
    pub to_key_block: bool,
    pub from: BlockIdExt,
    pub to: BlockIdExt,
    pub dest_proof: String,
    pub proof: String,
    pub state_proof: String,
}

#[derive(Debug, Deserialize)]
pub struct ShardBlockProofEntry {
    #[serde(rename = "@type")]
    pub type_field: String,
    pub to_key_block: bool,
    pub from: BlockIdExt,
    pub to: BlockIdExt,
    pub dest_proof: String,
    pub proof: String,
    pub state_proof: String,
}

#[derive(Debug, Deserialize)]
pub struct ConsensusBlock {
    pub consensus_block: u32,
    pub timestamp: f64,
}

#[derive(Debug, Deserialize)]
pub struct ShardsResult {
    #[serde(rename = "@type")]
    pub type_field: String,
    pub shards: Vec<BlockIdExt>,
    #[serde(rename = "@extra")]
    pub extra: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ShortTxId {
    #[serde(rename = "@type")]
    pub type_field: String,
    pub mode: i32,
    pub account: String,
    pub lt: String,
    pub hash: String,
}

#[derive(Debug, Deserialize)]
pub struct BlockTransactions {
    #[serde(rename = "@type")]
    pub type_field: String,
    pub id: BlockIdExt,
    pub req_count: u32,
    pub incomplete: bool,
    pub transactions: Vec<ShortTxId>,
    #[serde(rename = "@extra")]
    pub extra: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct BlockHeader {
    #[serde(rename = "@type")]
    pub type_field: String,
    pub id: BlockIdExt,
    pub global_id: i32,
    pub version: u32,
    pub flags: u32,
    pub after_merge: bool,
    pub after_split: bool,
    pub before_split: bool,
    pub want_merge: bool,
    pub want_split: bool,
    pub validator_list_hash_short: u32,
    pub catchain_seqno: u32,
    pub min_ref_mc_seqno: u32,
    pub is_key_block: bool,
    pub prev_key_block_seqno: u32,
    pub start_lt: String,
    pub end_lt: String,
    pub gen_utime: u64,
    pub prev_blocks: Vec<BlockIdExt>,
    #[serde(rename = "@extra")]
    pub extra: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TryLocateTxResponse {
    #[serde(rename = "@type")]
    pub type_field: String,
    pub address: AccountAddress,
    pub utime: u64,
    pub data: String,
    pub transaction_id: TransactionId,
    pub fee: String,
    pub storage_fee: String,
    pub other_fee: String,
    pub in_msg: Message,
    pub out_msgs: Vec<Message>,
}

#[derive(Debug, Deserialize)]
pub struct ConfigParamResponse {
    #[serde(rename = "@type")]
    pub type_field: String,
    pub config: TvmCell,
    #[serde(rename = "@extra")]
    pub extra: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TvmCell {
    #[serde(rename = "@type")]
    pub type_field: String,
    pub bytes: String,
}

#[derive(Debug, Deserialize)]
pub struct RunGetMethodResponse {
    #[serde(rename = "@type")]
    pub type_field: String,
    pub gas_used: u32,
    pub stack: Vec<(String, String)>,
    pub exit_code: i32,
    #[serde(rename = "@extra")]
    pub extra: Option<String>,
}
