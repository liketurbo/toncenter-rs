use super::base::Network;
use crate::client::base::{ApiKey, BaseApiClient};
use crate::error::ToncenterError;
use crate::models::{
    BlocksHeader, BlocksMasterchainInfo, BlocksShardBlockProof, BlocksShards, BlocksTransactions,
    ConfigInfo, ConsensusBlock, DetectAddressResult, FullAccountState, MasterchainBlockSignatures,
    QueryFees, RawExtMessageInfo, RawFullAccountState, RawTransaction, SmcRunResult, Success,
    TokenData, TonBlockIdExt, WalletInformation,
};

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
    /// * `address` - Identifier of the target TON account in any form.
    pub async fn get_address_information(
        &self,
        address: &str,
    ) -> Result<RawFullAccountState, ToncenterError> {
        let params = [("address", address)];

        self.base_client
            .get(&self.base_url, "getAddressInformation", &params)
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
    /// * `address` - Identifier of the target TON account in any form.
    pub async fn get_extended_address_information(
        &self,
        address: &str,
    ) -> Result<FullAccountState, ToncenterError> {
        let params = [("address", address)];

        self.base_client
            .get(&self.base_url, "getExtendedAddressInformation", &params)
            .await
    }

    /// Retrieve wallet information.
    ///
    /// This method parses contract state and currently supports more wallet types than getExtendedAddressInformation:
    /// simple wallet, standard wallet, v3 wallet, v4 wallet.
    ///
    /// # Parameters
    ///
    /// * `address` - Identifier of the target TON account in any form.
    pub async fn get_wallet_information(
        &self,
        address: &str,
    ) -> Result<WalletInformation, ToncenterError> {
        let params = [("address", address)];

        self.base_client
            .get(&self.base_url, "getWalletInformation", &params)
            .await
    }

    /// Get transaction history of a given address.
    ///
    /// # Parameters
    ///
    /// * `address` - Identifier of the target TON account in any form.
    /// * `limit` - Maximum number of transactions in response (optional).
    /// * `lt` - Logical time of transaction to start with, must be sent with `hash` (optional).
    /// * `hash` - Hash of transaction to start with, in `base64` or `hex` encoding, must be sent with `lt` (optional).
    /// * `to_lt` - Logical time of transaction to finish with (optional).
    /// * `archival` - If `true`, only liteservers with full history are used (optional).
    pub async fn get_transactions(
        &self,
        address: &str,
        limit: Option<u32>,
        lt: Option<u64>,
        hash: Option<&str>,
        to_lt: Option<u64>,
        archival: Option<bool>,
    ) -> Result<Vec<RawTransaction>, ToncenterError> {
        let mut params: Vec<(&str, String)> = vec![("address", address.to_string())];

        if let Some(limit) = limit {
            params.push(("limit", limit.to_string()));
        }
        if let Some(lt) = lt {
            params.push(("lt", lt.to_string()));
        }
        if let Some(hash) = hash {
            params.push(("hash", hash.to_string()));
        }
        if let Some(to_lt) = to_lt {
            params.push(("to_lt", to_lt.to_string()));
        }
        if let Some(archival) = archival {
            params.push(("archival", archival.to_string()));
        }

        let params: Vec<(&str, &str)> = params.iter().map(|(k, v)| (*k, v.as_str())).collect();

        self.base_client
            .get(&self.base_url, "getTransactions", &params)
            .await
    }

    /// Get balance (in nanotons) of a given address.
    ///
    /// # Parameters
    ///
    /// * `address` - Identifier of the target TON account in any form.
    pub async fn get_address_balance(&self, address: &str) -> Result<String, ToncenterError> {
        let params = [("address", address)];

        self.base_client
            .get(&self.base_url, "getAddressBalance", &params)
            .await
    }

    /// Get state of a given address. State can be either unitialized, active or frozen.
    ///
    /// # Parameters
    ///
    /// * `address` - Identifier of the target TON account in any form.
    pub async fn get_address_state(&self, address: &str) -> Result<String, ToncenterError> {
        let params = [("address", address)];

        self.base_client
            .get(&self.base_url, "getAddressState", &params)
            .await
    }

    /// Packs a raw address into a human-readable format.
    ///
    /// # Parameters
    ///
    /// * `address` - Identifier of the target TON account in raw form.
    pub async fn pack_address(&self, address: &str) -> Result<String, ToncenterError> {
        let params = [("address", address)];

        self.base_client
            .get(&self.base_url, "packAddress", &params)
            .await
    }

    /// Unpacks a human-readable address into its raw format.
    ///
    /// # Parameters
    ///
    /// * `address` - Identifier of the target TON account in user-friendly form.
    pub async fn unpack_address(&self, address: &str) -> Result<String, ToncenterError> {
        let params = [("address", address)];

        self.base_client
            .get(&self.base_url, "unpackAddress", &params)
            .await
    }

    /// Get NFT or Jetton information.
    ///
    /// # Parameters
    ///
    /// * `address` - Address of NFT collection/item or Jetton master/wallet smart contract.
    pub async fn get_token_data(&self, address: &str) -> Result<TokenData, ToncenterError> {
        let params = [("address", address)];

        self.base_client
            .get(&self.base_url, "getTokenData", &params)
            .await
    }

    /// Detect address in all possible forms.
    ///
    /// # Parameters
    ///
    /// * `address` - Identifier of the target TON account in any form.
    pub async fn detect_address(
        &self,
        address: &str,
    ) -> Result<DetectAddressResult, ToncenterError> {
        let params = [("address", address)];

        self.base_client
            .get(&self.base_url, "detectAddress", &params)
            .await
    }

    /// Get up-to-date masterchain state.
    ///
    /// # Parameters
    ///
    /// * `address` - Identifier of the target TON account in any form.
    pub async fn get_masterchain_info(&self) -> Result<BlocksMasterchainInfo, ToncenterError> {
        self.base_client
            .get(&self.base_url, "getMasterchainInfo", &[])
            .await
    }

    /// Get masterchain block signatures by sequence number.
    ///
    /// # Parameters
    ///
    /// * `seqno` - Sequence number of the masterchain block.
    pub async fn get_masterchain_block_signatures(
        &self,
        seqno: u32,
    ) -> Result<MasterchainBlockSignatures, ToncenterError> {
        let seqno_string = seqno.to_string();
        let seqno_ref = seqno_string.as_str();

        let params = [("seqno", seqno_ref)];

        self.base_client
            .get(&self.base_url, "getMasterchainBlockSignatures", &params)
            .await
    }

    /// Get shard block proof.
    ///
    /// # Parameters
    ///
    /// * `workchain` - Block workchain id.
    /// * `shard` - Block shard id.
    /// * `seqno` - Block seqno.
    /// * `from_seqno` - Seqno of masterchain block starting from which proof is required. Optional.
    pub async fn get_shard_block_proof(
        &self,
        workchain: i32,
        shard: &str,
        seqno: u32,
        from_seqno: Option<u32>,
    ) -> Result<BlocksShardBlockProof, ToncenterError> {
        let mut params: Vec<(&str, String)> = vec![
            ("workchain", workchain.to_string()),
            ("shard", shard.to_string()),
            ("seqno", seqno.to_string()),
        ];

        if let Some(from_seqno) = from_seqno {
            params.push(("from_seqno", from_seqno.to_string()));
        }

        let params: Vec<(&str, &str)> = params.iter().map(|(k, v)| (*k, v.as_str())).collect();

        self.base_client
            .get(&self.base_url, "getShardBlockProof", &params)
            .await
    }

    /// Get consensus block and its update timestamp.
    pub async fn get_consensus_block(&self) -> Result<ConsensusBlock, ToncenterError> {
        self.base_client
            .get(&self.base_url, "getConsensusBlock", &[])
            .await
    }

    /// Look up block by either seqno, lt or unixtime.
    ///
    /// # Parameters
    ///
    /// * `workchain` - Workchain id to look up block in.
    /// * `shard` - Shard id to look up block in.
    /// * `seqno` - Block's height (optional).
    /// * `lt` - Block's logical time (optional).
    /// * `unixtime` - Block's unixtime (optional).
    pub async fn lookup_block(
        &self,
        workchain: i32,
        shard: &str,
        seqno: Option<u32>,
        lt: Option<u64>,
        unixtime: Option<u64>,
    ) -> Result<TonBlockIdExt, ToncenterError> {
        let mut params: Vec<(&str, String)> = vec![
            ("workchain", workchain.to_string()),
            ("shard", shard.to_string()),
        ];

        if let Some(seqno) = seqno {
            params.push(("seqno", seqno.to_string()));
        }
        if let Some(lt) = lt {
            params.push(("lt", lt.to_string()));
        }
        if let Some(unixtime) = unixtime {
            params.push(("unixtime", unixtime.to_string()));
        }

        let params: Vec<(&str, &str)> = params.iter().map(|(k, v)| (*k, v.as_str())).collect();

        self.base_client
            .get(&self.base_url, "lookupBlock", &params)
            .await
    }

    /// Get shards information.
    ///
    /// # Parameters
    ///
    /// * `seqno` - Masterchain seqno to fetch shards of.
    pub async fn get_shards(&self, seqno: u32) -> Result<BlocksShards, ToncenterError> {
        let seqno_string = seqno.to_string();
        let seqno_ref = seqno_string.as_str();

        let params = [("seqno", seqno_ref)];

        self.base_client
            .get(&self.base_url, "shards", &params)
            .await
    }

    /// Get transactions of the given block.
    ///
    /// # Parameters
    ///
    /// * `workchain` - Workchain id to look up block in.
    /// * `shard` - Shard id to look up block in.
    /// * `seqno` - Block's height.
    /// * `root_hash` - Block's root hash (optional).
    /// * `file_hash` - Block's file hash (optional).
    /// * `after_lt` - Logical time of transaction after which to start (optional).
    /// * `after_hash` - Hash of transaction after which to start (optional).
    /// * `count` - Maximum number of transactions to return (optional, default is 40).
    pub async fn get_block_transactions(
        &self,
        workchain: i32,
        shard: &str,
        seqno: u32,
        root_hash: Option<&str>,
        file_hash: Option<&str>,
        after_lt: Option<u64>,
        after_hash: Option<&str>,
        count: Option<u32>,
    ) -> Result<BlocksTransactions, ToncenterError> {
        let mut params: Vec<(&str, String)> = vec![
            ("workchain", workchain.to_string()),
            ("shard", shard.to_string()),
            ("seqno", seqno.to_string()),
        ];

        if let Some(root_hash) = root_hash {
            params.push(("root_hash", root_hash.to_string()));
        }
        if let Some(file_hash) = file_hash {
            params.push(("file_hash", file_hash.to_string()));
        }
        if let Some(after_lt) = after_lt {
            params.push(("after_lt", after_lt.to_string()));
        }
        if let Some(after_hash) = after_hash {
            params.push(("after_hash", after_hash.to_string()));
        }
        if let Some(count) = count {
            params.push(("count", count.to_string()));
        }

        let params: Vec<(&str, &str)> = params.iter().map(|(k, v)| (*k, v.as_str())).collect();

        self.base_client
            .get(&self.base_url, "getBlockTransactions", &params)
            .await
    }

    /// Get metadata of a given block.
    ///
    /// # Parameters
    ///
    /// * `workchain` - Workchain id to look up block in.
    /// * `shard` - Shard id to look up block in.
    /// * `seqno` - Block's height.
    /// * `root_hash` - Block's root hash (optional).
    /// * `file_hash` - Block's file hash (optional).
    pub async fn get_block_header(
        &self,
        workchain: i32,
        shard: &str,
        seqno: u32,
        root_hash: Option<&str>,
        file_hash: Option<&str>,
    ) -> Result<BlocksHeader, ToncenterError> {
        let mut params: Vec<(&str, String)> = vec![
            ("workchain", workchain.to_string()),
            ("shard", shard.to_string()),
            ("seqno", seqno.to_string()),
        ];

        if let Some(root_hash) = root_hash {
            params.push(("root_hash", root_hash.to_string()));
        }
        if let Some(file_hash) = file_hash {
            params.push(("file_hash", file_hash.to_string()));
        }

        let params: Vec<(&str, &str)> = params.iter().map(|(k, v)| (*k, v.as_str())).collect();

        self.base_client
            .get(&self.base_url, "getBlockHeader", &params)
            .await
    }

    /// Locate outgoing transaction of destination address by incoming message.
    ///
    /// # Parameters
    ///
    /// * `source` - Source address.
    /// * `destination` - Destination address.
    /// * `created_lt` - Created logical time.
    pub async fn try_locate_tx(
        &self,
        source: &str,
        destination: &str,
        created_lt: u64,
    ) -> Result<RawTransaction, ToncenterError> {
        let params = [
            ("source", source),
            ("destination", destination),
            ("created_lt", &created_lt.to_string()),
        ];

        self.base_client
            .get(&self.base_url, "tryLocateTx", &params)
            .await
    }

    /// Same as previous. Locate outgoing transaction of destination address by incoming message.
    ///
    /// # Parameters
    ///
    /// * `source` - Source address.
    /// * `destination` - Destination address.
    /// * `created_lt` - Created logical time.
    pub async fn try_locate_result_tx(
        &self,
        source: &str,
        destination: &str,
        created_lt: u64,
    ) -> Result<RawTransaction, ToncenterError> {
        let params = [
            ("source", source),
            ("destination", destination),
            ("created_lt", &created_lt.to_string()),
        ];

        self.base_client
            .get(&self.base_url, "tryLocateResultTx", &params)
            .await
    }

    /// Locate incoming transaction of source address by outgoing message.
    ///
    /// # Parameters
    ///
    /// * `source` - Source address.
    /// * `destination` - Destination address.
    /// * `created_lt` - Created logical time.
    pub async fn try_locate_source_tx(
        &self,
        source: &str,
        destination: &str,
        created_lt: u64,
    ) -> Result<RawTransaction, ToncenterError> {
        let params = [
            ("source", source),
            ("destination", destination),
            ("created_lt", &created_lt.to_string()),
        ];

        self.base_client
            .get(&self.base_url, "tryLocateSourceTx", &params)
            .await
    }

    /// Get config parameter by id.
    ///
    /// # Parameters
    ///
    /// * `config_id` - Configuration id.
    /// * `seqno` - Masterchain seqno (optional). If not specified, latest blockchain state will be used.
    pub async fn get_config_param(
        &self,
        config_id: u32,
        seqno: Option<u32>,
    ) -> Result<ConfigInfo, ToncenterError> {
        let mut params: Vec<(&str, String)> = vec![("config_id", config_id.to_string())];

        if let Some(seqno) = seqno {
            params.push(("seqno", seqno.to_string()));
        }

        let params: Vec<(&str, &str)> = params.iter().map(|(k, v)| (*k, v.as_str())).collect();

        self.base_client
            .get(&self.base_url, "getConfigParam", &params)
            .await
    }

    /// Run get method on smart contract.
    ///
    /// # Parameters
    ///
    /// * `address` - Address of the smart contract.
    /// * `method` - Method name to run.
    /// * `params` - Parameters for the method.
    pub async fn run_get_method(
        &self,
        address: &str,
        method: &str,
        stack: &[&str],
    ) -> Result<SmcRunResult, ToncenterError> {
        let request_body = serde_json::json!({
            "address": address,
            "method": method,
            "stack": stack
        });

        self.base_client
            .post(&self.base_url, "runGetMethod", &request_body)
            .await
    }

    /// Send serialized BOC file: fully packed and serialized external message to blockchain.
    ///
    /// # Parameters
    ///
    /// * `boc` - Serialized BOC file (b64-encoded).
    pub async fn send_boc(&self, boc: &str) -> Result<Success, ToncenterError> {
        let body = serde_json::json!({
            "boc": boc,
        });
        self.base_client
            .post(&self.base_url, "sendBoc", &body)
            .await
    }

    /// Send serialized BOC file: fully packed and serialized external message to blockchain.
    /// The method returns message hash.
    ///
    /// # Parameters
    ///
    /// * `boc` - Serialized BOC file (b64-encoded).
    pub async fn send_boc_return_hash(
        &self,
        boc: &str,
    ) -> Result<RawExtMessageInfo, ToncenterError> {
        let body = serde_json::json!({
            "boc": boc,
        });
        self.base_client
            .post(&self.base_url, "sendBocReturnHash", &body)
            .await
    }

    /// This method takes address, body and init-params (if any), packs it to external message and sends to network.
    /// All params should be BOC-serialized.
    ///
    /// # Parameters
    ///
    /// * `address` - The target address.
    /// * `body` - Optional BOC-serialized body (b64-encoded).
    /// * `init_code` - Optional BOC-serialized init code (b64-encoded).
    /// * `init_data` - Optional BOC-serialized init data (b64-encoded).
    ///
    /// # Returns
    ///
    /// A `Result` containing `SendQueryResponse` on success, or `ToncenterError` on failure.
    pub async fn send_query(
        &self,
        address: &str,
        body: Option<&str>,
        init_code: Option<&str>,
        init_data: Option<&str>,
    ) -> Result<Success, ToncenterError> {
        let mut request_body = serde_json::json!({
            "address": address,
        });

        if let Some(b) = body {
            request_body["body"] = serde_json::json!(b);
        }
        if let Some(code) = init_code {
            request_body["init_code"] = serde_json::json!(code);
        }
        if let Some(data) = init_data {
            request_body["init_data"] = serde_json::json!(data);
        }

        self.base_client
            .post(&self.base_url, "sendQuery", &request_body)
            .await
    }

    /// Estimate fees required for query processing.
    /// `body`, `init_code`, and `init_data` accepted in serialized format (b64-encoded).
    ///
    /// # Parameters
    ///
    /// * `address` - The target address.
    /// * `body` - Optional BOC-serialized body (b64-encoded).
    /// * `init_code` - Optional BOC-serialized init code (b64-encoded).
    /// * `init_data` - Optional BOC-serialized init data (b64-encoded).
    ///
    /// # Returns
    ///
    /// A `Result` containing `EstimateFeeResponse` on success, or `ToncenterError` on failure.
    pub async fn estimate_fee(
        &self,
        address: &str,
        body: Option<&str>,
        init_code: Option<&str>,
        init_data: Option<&str>,
        ignore_chksig: Option<bool>,
    ) -> Result<QueryFees, ToncenterError> {
        let mut request_body = serde_json::json!({
            "address": address,
        });

        if let Some(b) = body {
            request_body["body"] = serde_json::json!(b);
        }
        if let Some(code) = init_code {
            request_body["init_code"] = serde_json::json!(code);
        }
        if let Some(data) = init_data {
            request_body["init_data"] = serde_json::json!(data);
        }
        if let Some(chksig) = ignore_chksig {
            request_body["ignore_chksig"] = serde_json::json!(chksig);
        }

        self.base_client
            .post(&self.base_url, "estimateFee", &request_body)
            .await
    }
}
