use toncenter_anychain::client::{ApiClientV2, ApiKey, Network};

#[tokio::main]
async fn main() {
    env_logger::init();

    let api_key = "a8b61ced4be11488cb6e82d65b93e3d4a29d20af406aed9688b9e0077e2dc742".to_string();
    let address = "0QCbOix87iy37AwRCWaYhJHzc2gXE_WnAG5vVEAySNT7zClz";

    let api_client = ApiClientV2::new(Network::Testnet, Some(ApiKey::Header(api_key)));

    match api_client.get_address_information(address).await {
        Ok(info) => println!("Address info: {:#?}", info),
        Err(e) => {
            eprintln!("{:?}", e);
        }
    }

    let params = serde_json::json!({
        "address": address,
    });

    match api_client
        .json_rpc("getAddressInformation", params, serde_json::json!(1))
        .await
    {
        Ok(response) => println!("Response: {:#?}", response),
        Err(e) => {
            eprintln!("{:?}", e);
        }
    };

    match api_client.run_get_method(address, "seqno", &[]).await {
        Ok(info) => {
            if info.exit_code == 0 {
                let (_type, hex_value) = info.stack.first().unwrap();
                let seqno: u64 =
                    u64::from_str_radix(hex_value.trim_start_matches("0x"), 16).unwrap();
                println!("{}", seqno);
            }
        }
        Err(e) => {
            eprintln!("{:?}", e);
        }
    }
}
