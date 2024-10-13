use serde::Deserialize;

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

#[cfg(test)]
mod tests {
    use super::ApiResponseResultV3;
    use crate::models_v3::RawFullAccountStateV3;
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
}
