use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ApiResponseResultV3<T> {
    Success(T),
    Error { error: Option<String> },
}

/// Represents `AccountStateFull`.
#[derive(Debug, Deserialize)]
pub struct RawFullAccountStateV3 {
    pub balance: String,
    pub code: Option<String>,
    pub data: Option<String>,
    #[serde(flatten)]
    pub last_transaction_id: InternalTransactionIdV3,
    pub frozen_hash: Option<String>,
    pub status: String,
}

/// Represents `transactionId with last_transaction_hash and last_transaction_lt`.
#[derive(Debug, Deserialize)]
pub struct InternalTransactionIdV3 {
    pub last_transaction_hash: String,
    pub last_transaction_lt: String,
}

/// Represents `Send an external message to the TON network successful response`.
#[derive(Debug, Deserialize)]
pub struct MessageSuccessResponse {
    pub message_hash: String,
}

/// Represents `smc.runResult`.
#[derive(Debug, Serialize, Deserialize)]
pub struct SmcRunResult {
    pub gas_used: u32,
    pub stack: Vec<SmcRunResultStackTypeValue>,
    pub exit_code: i32,
    pub extra: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SmcRunResultStackTypeValue {
    #[serde(rename = "type")]
    pub the_type: String,
    pub value: String,
}

use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct JettonWallet {
    pub address: String,
    pub balance: String,
    pub owner: String,
    pub jetton: String,
    pub last_transaction_lt: String,
    pub code_hash: String,
    pub data_hash: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddressBookEntry {
    pub user_friendly: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JettonWalletsResponse {
    pub jetton_wallets: Vec<JettonWallet>,
    pub address_book: HashMap<String, AddressBookEntry>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    #[test]
    fn test_address_information_decode() {
        let response = json!({
          "balance": "1609999315",
          "code": "te6cckECFAEAAtQAART/APSkE/S88sgLAQIBIAIDAgFIBAUE+PKDCNcYINMf0x/THwL4I7vyZO1E0NMf0x/T//QE0VFDuvKhUVG68qIF+QFUEGT5EPKj+AAkpMjLH1JAyx9SMMv/UhD0AMntVPgPAdMHIcAAn2xRkyDXSpbTB9QC+wDoMOAhwAHjACHAAuMAAcADkTDjDQOkyMsfEssfy/8QERITAubQAdDTAyFxsJJfBOAi10nBIJJfBOAC0x8hghBwbHVnvSKCEGRzdHK9sJJfBeAD+kAwIPpEAcjKB8v/ydDtRNCBAUDXIfQEMFyBAQj0Cm+hMbOSXwfgBdM/yCWCEHBsdWe6kjgw4w0DghBkc3RyupJfBuMNBgcCASAICQB4AfoA9AQw+CdvIjBQCqEhvvLgUIIQcGx1Z4MesXCAGFAEywUmzxZY+gIZ9ADLaRfLH1Jgyz8gyYBA+wAGAIpQBIEBCPRZMO1E0IEBQNcgyAHPFvQAye1UAXKwjiOCEGRzdHKDHrFwgBhQBcsFUAPPFiP6AhPLassfyz/JgED7AJJfA+ICASAKCwBZvSQrb2omhAgKBrkPoCGEcNQICEekk30pkQzmkD6f+YN4EoAbeBAUiYcVnzGEAgFYDA0AEbjJftRNDXCx+AA9sp37UTQgQFA1yH0BDACyMoHy//J0AGBAQj0Cm+hMYAIBIA4PABmtznaiaEAga5Drhf/AABmvHfaiaEAQa5DrhY/AAG7SB/oA1NQi+QAFyMoHFcv/ydB3dIAYyMsFywIizxZQBfoCFMtrEszMyXP7AMhAFIEBCPRR8qcCAHCBAQjXGPoA0z/IVCBHgQEI9FHyp4IQbm90ZXB0gBjIywXLAlAGzxZQBPoCFMtqEssfyz/Jc/sAAgBsgQEI1xj6ANM/MFIkgQEI9Fnyp4IQZHN0cnB0gBjIywXLAlAFzxZQA/oCE8tqyx8Syz/Jc/sAAAr0AMntVGliJeU=",
          "data": "te6cckEBAQEAKwAAUQAAABIpqaMX/w9EG1j/2P4YLDtKl+AbrUrXdNAUrgL5lgIIz3rupJBAVNMvcg==",
          "last_transaction_lt": "26848796000001",
          "last_transaction_hash": "QHUTlAQ3QYFqPfgU0YQ3CWZnaJeUyWyvASZcXBkiHP8=",
          "frozen_hash": null,
          "status": "active"
        });

        let result: Result<ApiResponseResultV3<RawFullAccountStateV3>, _> =
            serde_json::from_value(response);
        assert!(result.is_ok());
    }

    #[test]
    fn test_send_message_decode() {
        let response = json!({
          "message_hash": "2LhE9RZPDXFdjRYAHyN5BBalWXMOuyqgMVwr0V/+0GA=",
        });

        let result: Result<ApiResponseResultV3<MessageSuccessResponse>, _> =
            serde_json::from_value(response);
        assert!(result.is_ok());
    }

    #[test]
    fn test_rungetmethod_seqno_decode() {
        /*
        use super::SmcRunResult;
        let smc = SmcRunResult {
            gas_used: 769,
            stack: vec![SmcRunResultStackTypeValue {
                the_type: "num".to_string(),
                value: "0x14".to_string(),
            }],
            exit_code: 0,
            extra: None,
        };
        let smc_json = serde_json::to_string(&smc).unwrap();
         */

        let response = json!({
            "gas_used":769,
            "exit_code":0,
            "stack":[{"type":"num","value":"0x14"}]
        });

        let result: Result<ApiResponseResultV3<SmcRunResult>, _> = serde_json::from_value(response);
        assert!(result.is_ok());
        let smc = result.unwrap();
        matches!(smc, ApiResponseResultV3::Success(SmcRunResult { .. }));
    }

    #[test]
    fn test_get_jetton_wallets() {
        let response = json!({
            "jetton_wallets":[{
                "address":"0:48FA147B278E22D7FE26C9C7D449999AC929CB818B3BC7A032E5988E73576EB6",
                "balance":"20000030000000000",
                "owner":"0:3A5B6B29449E83F8051FE8F1D37CA24289BEB764953A7CAF01C47569DF6D9495",
                "jetton":"0:BA3F638CF1EB9D832B8F37A2C7980CCBE41F2CDFEC6DC9B57E147944C78FF284",
                "last_transaction_lt":"26848793000003",
                "code_hash":"vrBoPr64kn/p/I7AoYvH3ReJlomCWhIeq0bFo6hg0M4=",
                "data_hash":"4YqZpb2TbYp4pVDInH/2aqQRtRbos1YkxbE5SQyhh00="
              }],
           "address_book":{
                "0:3A5B6B29449E83F8051FE8F1D37CA24289BEB764953A7CAF01C47569DF6D9495":{
                "user_friendly":"0QA6W2spRJ6D-AUf6PHTfKJCib63ZJU6fK8BxHVp322UlXe4"
              },
              "0:48FA147B278E22D7FE26C9C7D449999AC929CB818B3BC7A032E5988E73576EB6":{
                 "user_friendly":"kQBI-hR7J44i1_4mycfUSZmaySnLgYs7x6Ay5ZiOc1dutqkJ"
              },
              "0:BA3F638CF1EB9D832B8F37A2C7980CCBE41F2CDFEC6DC9B57E147944C78FF284":{
                 "user_friendly":"kQC6P2OM8eudgyuPN6LHmAzL5B8s3-xtybV-FHlEx4_yhKBN"
              }
           }
        });

        let result: Result<ApiResponseResultV3<JettonWalletsResponse>, _> =
            serde_json::from_value(response);
        assert!(result.is_ok());
        matches!(
            result.unwrap(),
            ApiResponseResultV3::Success(JettonWalletsResponse { .. })
        );
    }
}
