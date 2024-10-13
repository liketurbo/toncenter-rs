use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApiResponseResult<T> {
    Success(T),
    Error { error: Option<String> },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MockAccountState {
    pub balance: String,
    pub code: String,
    pub data: String,
    pub last_transaction_lt: String,
    pub last_transaction_hash: String,
    pub frozen_hash: Option<String>,
    pub status: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_encode_succ_response() {
        let account_state = MockAccountState {
            balance: "1561185728".to_string(),
            code: "te6cckECFAEAAtQAART/APSkE/S88sgLAQIBIAIDAgFIBAUE+PKDCNcYINMf0x/THwL4I7vyZO1E0NMf0x/T//QE0VFDuvKhUVG68qIF+QFUEGT5EPKj+AAkpMjLH1JAyx9SMMv/UhD0AMntVPgPAdMHIcAAn2xRkyDXSpbTB9QC+wDoMOAhwAHjACHAAuMAAcADkTDjDQOkyMsfEssfy/8QERITAubQAdDTAyFxsJJfBOAi10nBIJJfBOAC0x8hghBwbHVnvSKCEGRzdHK9sJJfBeAD+kAwIPpEAcjKB8v/ydDtRNCBAUDXIfQEMFyBAQj0Cm+hMbOSXwfgBdM/yCWCEHBsdWe6kjgw4w0DghBkc3RyupJfBuMNBgcCASAICQB4AfoA9AQw+CdvIjBQCqEhvvLgUIIQcGx1Z4MesXCAGFAEywUmzxZY+gIZ9ADLaRfLH1Jgyz8gyYBA+wAGAIpQBIEBCPRZMO1E0IEBQNcgyAHPFvQAye1UAXKwjiOCEGRzdHKDHrFwgBhQBcsFUAPPFiP6AhPLassfyz/JgED7AJJfA+ICASAKCwBZvSQrb2omhAgKBrkPoCGEcNQICEekk30pkQzmkD6f+YN4EoAbeBAUiYcVnzGEAgFYDA0AEbjJftRNDXCx+AA9sp37UTQgQFA1yH0BDACyMoHy//J0AGBAQj0Cm+hMYAIBIA4PABmtznaiaEAga5Drhf/AABmvHfaiaEAQa5DrhY/AAG7SB/oA1NQi+QAFyMoHFcv/ydB3dIAYyMsFywIizxZQBfoCFMtrEszMyXP7AMhAFIEBCPRR8qcCAHCBAQjXGPoA0z/IVCBHgQEI9FHyp4IQbm90ZXB0gBjIywXLAlAGzxZQBPoCFMtqEssfyz/Jc/sAAgBsgQEI1xj6ANM/MFIkgQEI9Fnyp4IQZHN0cnB0gBjIywXLAlAFzxZQA/oCE8tqyx8Syz/Jc/sAAAr0AMntVGliJeU=".to_string(),
            data: "te6cckEBAQEAKwAAUQAAAAcpqaMX/w9EG1j/2P4YLDtKl+AbrUrXdNAUrgL5lgIIz3rupJBAwVw5FQ==".to_string(),
            last_transaction_lt: "26225862000001".to_string(),
            last_transaction_hash: "eDEE0kLRHXXCadU/EnzzwreoVdEFOqJfbXE4FBma4U4=".to_string(),
            frozen_hash: None,
            status: "active".to_string(),
        };
        let response = ApiResponseResult::Success(account_state);
        let encoded_json = serde_json::to_value(&response);
        assert!(encoded_json.is_ok());
    }

    #[test]
    fn test_decode_succ_response() {
        let response = json!({
            "balance": "1561185728",
            "code": "te6cckECFAEAAtQAART/APSkE/S88sgLAQIBIAIDAgFIBAUE+PKDCNcYINMf0x/THwL4I7vyZO1E0NMf0x/T//QE0VFDuvKhUVG68qIF+QFUEGT5EPKj+AAkpMjLH1JAyx9SMMv/UhD0AMntVPgPAdMHIcAAn2xRkyDXSpbTB9QC+wDoMOAhwAHjACHAAuMAAcADkTDjDQOkyMsfEssfy/8QERITAubQAdDTAyFxsJJfBOAi10nBIJJfBOAC0x8hghBwbHVnvSKCEGRzdHK9sJJfBeAD+kAwIPpEAcjKB8v/ydDtRNCBAUDXIfQEMFyBAQj0Cm+hMbOSXwfgBdM/yCWCEHBsdWe6kjgw4w0DghBkc3RyupJfBuMNBgcCASAICQB4AfoA9AQw+CdvIjBQCqEhvvLgUIIQcGx1Z4MesXCAGFAEywUmzxZY+gIZ9ADLaRfLH1Jgyz8gyYBA+wAGAIpQBIEBCPRZMO1E0IEBQNcgyAHPFvQAye1UAXKwjiOCEGRzdHKDHrFwgBhQBcsFUAPPFiP6AhPLassfyz/JgED7AJJfA+ICASAKCwBZvSQrb2omhAgKBrkPoCGEcNQICEekk30pkQzmkD6f+YN4EoAbeBAUiYcVnzGEAgFYDA0AEbjJftRNDXCx+AA9sp37UTQgQFA1yH0BDACyMoHy//J0AGBAQj0Cm+hMYAIBIA4PABmtznaiaEAga5Drhf/AABmvHfaiaEAQa5DrhY/AAG7SB/oA1NQi+QAFyMoHFcv/ydB3dIAYyMsFywIizxZQBfoCFMtrEszMyXP7AMhAFIEBCPRR8qcCAHCBAQjXGPoA0z/IVCBHgQEI9FHyp4IQbm90ZXB0gBjIywXLAlAGzxZQBPoCFMtqEssfyz/Jc/sAAgBsgQEI1xj6ANM/MFIkgQEI9Fnyp4IQZHN0cnB0gBjIywXLAlAFzxZQA/oCE8tqyx8Syz/Jc/sAAAr0AMntVGliJeU=",
            "data": "te6cckEBAQEAKwAAUQAAAAcpqaMX/w9EG1j/2P4YLDtKl+AbrUrXdNAUrgL5lgIIz3rupJBAwVw5FQ==",
            "last_transaction_lt": "26225862000001",
            "last_transaction_hash": "eDEE0kLRHXXCadU/EnzzwreoVdEFOqJfbXE4FBma4U4=",
            "frozen_hash": null,
            "status": "active"
        });
        // Decode the successful response
        let result: ApiResponseResult<MockAccountState> = serde_json::from_value(response).unwrap();
        // dbg!(result);
        matches!(result, ApiResponseResult::Success(MockAccountState { .. }));
    }

    #[test]
    fn test_decode_err_response() {
        let response = json!({
            "error": "failed to decode: schema: error converting value for \"address\""
        });
        let result: ApiResponseResult<MockAccountState> = serde_json::from_value(response).unwrap();
        // Decode the error response
        matches!(result, ApiResponseResult::Error { .. });
    }
}
