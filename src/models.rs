use serde::{Deserialize, Serialize};

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

/// Represents `@type: raw.fullAccountState`.
#[derive(Debug, Deserialize)]
pub struct RawFullAccountState {
    pub balance: String,
    pub code: Option<String>,
    pub data: Option<String>,
    pub last_transaction_id: InternalTransactionId,
    pub block_id: TonBlockIdExt,
    pub frozen_hash: Option<String>,
    pub sync_utime: u64,
    #[serde(rename = "@extra")]
    pub extra: String,
    pub state: String,
}

/// Represents `@type: fullAccountState`.
#[derive(Debug, Deserialize)]
pub struct FullAccountState {
    pub address: AccountAddress,
    pub balance: String,
    pub last_transaction_id: InternalTransactionId,
    pub block_id: TonBlockIdExt,
    pub sync_utime: u64,
    pub account_state: AccountState,
    pub revision: i32,
    #[serde(rename = "@extra")]
    pub extra: String,
}

/// Represents `@type: accountAddress`.
#[derive(Debug, Deserialize)]
pub struct AccountAddress {
    pub account_address: String,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "@type")]
pub enum AccountState {
    #[serde(rename = "uninited.accountState")]
    UninitedAccountState { frozen_hash: String },
    #[serde(rename = "wallet.v4.accountState")]
    WalletV4AccountState { wallet_id: String, seqno: u32 },
}

/// Represents `@type: internal.transactionId`.
#[derive(Debug, Deserialize)]
pub struct InternalTransactionId {
    pub lt: String,
    pub hash: String,
}

/// Represents `@type: ton.blockIdExt`.
#[derive(Debug, Deserialize)]
pub struct TonBlockIdExt {
    pub workchain: i32,
    pub shard: String,
    pub seqno: u32,
    pub root_hash: String,
    pub file_hash: String,
    #[serde(rename = "@extra")]
    pub extra: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct WalletInformation {
    pub wallet: bool,
    pub balance: String,
    pub account_state: String,
    pub wallet_type: String,
    pub seqno: u32,
    pub last_transaction_id: InternalTransactionId,
    pub wallet_id: u64,
}

/// Represents `@type: raw.transaction`.
#[derive(Debug, Deserialize)]
pub struct RawTransaction {
    pub address: AccountAddress,
    pub utime: u64,
    pub data: String,
    pub transaction_id: InternalTransactionId,
    pub fee: String,
    pub storage_fee: String,
    pub other_fee: String,
    pub in_msg: Option<RawMessage>,
    pub out_msgs: Vec<RawMessage>,
}

/// Represents `@type: raw.message`.
#[derive(Debug, Deserialize)]
pub struct RawMessage {
    pub source: Option<String>,
    pub destination: String,
    pub value: String,
    pub fwd_fee: String,
    pub ihr_fee: String,
    pub created_lt: String,
    pub body_hash: String,
    pub msg_data: MsgDataRaw,
    pub message: Option<String>,
}

/// Represents `@type: msg.dataRaw`.
#[derive(Debug, Deserialize)]
pub struct MsgDataRaw {
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

/// Represents `@type: blocks.masterchainInfo`.
#[derive(Debug, Deserialize)]
pub struct BlocksMasterchainInfo {
    pub last: TonBlockIdExt,
    pub state_root_hash: String,
    pub init: TonBlockIdExt,
    #[serde(rename = "@extra")]
    pub extra: String,
}

#[derive(Debug, Deserialize)]
pub struct MasterchainBlockSignatures {
    #[serde(rename = "@type")]
    pub type_field: String,
    pub id: TonBlockIdExt,
    pub signatures: Vec<BlocksSignature>,
    #[serde(rename = "@extra")]
    pub extra: String,
}

/// Represents `@type: blocks.signature`.
#[derive(Debug, Deserialize)]
pub struct BlocksSignature {
    pub node_id_short: String,
    pub signature: String,
}

/// Represents `@type: blocks.shardBlockProof`.
#[derive(Debug, Deserialize)]
pub struct BlocksShardBlockProof {
    pub from: TonBlockIdExt,
    pub mc_id: TonBlockIdExt,
    pub links: Vec<BlocksShardBlockLink>,
    pub mc_proof: Vec<BlocksBlockLinkBack>,
    #[serde(rename = "@extra")]
    pub extra: String,
}

/// Represents `@type: blocks.shardBlockLink`.
#[derive(Debug, Deserialize)]
pub struct BlocksShardBlockLink {
    pub id: TonBlockIdExt,
    pub proof: String,
}

/// Represents `@type: blocks.blockLinkBack`.
#[derive(Debug, Deserialize)]
pub struct BlocksBlockLinkBack {
    pub to_key_block: bool,
    pub from: TonBlockIdExt,
    pub to: TonBlockIdExt,
    pub dest_proof: String,
    pub proof: String,
    pub state_proof: String,
}

#[derive(Debug, Deserialize)]
pub struct ConsensusBlock {
    pub consensus_block: u32,
    pub timestamp: f64,
}

/// Represents `@type: blocks.shards`.
#[derive(Debug, Deserialize)]
pub struct BlocksShards {
    pub shards: Vec<TonBlockIdExt>,
    #[serde(rename = "@extra")]
    pub extra: Option<String>,
}

/// Represents `@type: blocks.transactions`.
#[derive(Debug, Deserialize)]
pub struct BlocksTransactions {
    pub id: TonBlockIdExt,
    pub req_count: u32,
    pub incomplete: bool,
    pub transactions: Vec<BlocksShortTxId>,
    #[serde(rename = "@extra")]
    pub extra: Option<String>,
}

/// Represents `@type: blocks.shortTxId`.
#[derive(Debug, Deserialize)]
pub struct BlocksShortTxId {
    pub mode: i32,
    pub account: String,
    pub lt: String,
    pub hash: String,
}

/// Represents `@type: blocks.header`.
#[derive(Debug, Deserialize)]
pub struct BlocksHeader {
    pub id: TonBlockIdExt,
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
    pub prev_blocks: Vec<TonBlockIdExt>,
    #[serde(rename = "@extra")]
    pub extra: Option<String>,
}

/// Represents `@type: configInfo`.
#[derive(Debug, Deserialize)]
pub struct ConfigInfo {
    pub config: TvmCell,
    #[serde(rename = "@extra")]
    pub extra: Option<String>,
}

/// Represents `@type: tvm.cell`.
#[derive(Debug, Deserialize)]
pub struct TvmCell {
    pub bytes: String,
}

/// Represents `@type: smc.runResult`.
#[derive(Debug, Deserialize)]
pub struct SmcRunResult {
    pub gas_used: u32,
    pub stack: Vec<(String, String)>,
    pub exit_code: i32,
    #[serde(rename = "@extra")]
    pub extra: Option<String>,
}

/// Represents `@type: ok`.
#[derive(Debug, Deserialize)]
pub struct Success {
    #[serde(rename = "@extra")]
    pub extra: String,
}

/// Represents `@type: raw.extMessageInfo`.
#[derive(Debug, Deserialize)]
pub struct RawExtMessageInfo {
    pub hash: String,
    #[serde(rename = "@extra")]
    pub extra: String,
}

/// Represents `@type: query.fees`.
#[derive(Debug, Deserialize)]
pub struct QueryFees {
    pub source_fees: Fees,
    pub destination_fees: Vec<Fees>,
    #[serde(rename = "@extra")]
    pub extra: String,
}

/// Represents `@type: fees`.
#[derive(Debug, Deserialize)]
pub struct Fees {
    pub in_fwd_fee: u64,
    pub storage_fee: u64,
    pub gas_fee: u64,
    pub fwd_fee: u64,
}

#[derive(Debug, Serialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub method: String,
    pub params: serde_json::Value,
    pub id: serde_json::Value,
}

#[derive(Debug, Deserialize)]
pub struct JsonRpcResponse<T> {
    pub ok: bool,
    pub jsonrpc: String,
    #[serde(flatten)]
    pub data: JsonRpcResult<T>,
    pub id: serde_json::Value,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum JsonRpcResult<T> {
    Success {
        result: T,
    },
    Error {
        result: Option<String>,
        error: Option<String>,
        code: u32,
    },
}
