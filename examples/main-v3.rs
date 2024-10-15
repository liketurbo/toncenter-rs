use toncenter::client::{ApiClientV3, ApiKey, Network};
#[tokio::main]
async fn main() {
    env_logger::init();

    let api_key = "a8b61ced4be11488cb6e82d65b93e3d4a29d20af406aed9688b9e0077e2dc742".to_string();
    let address = "0QA6W2spRJ6D-AUf6PHTfKJCib63ZJU6fK8BxHVp322UlXe4";

    let api_client = ApiClientV3::new(Network::Testnet, Some(ApiKey::Header(api_key)));

    match api_client.get_address_information(address).await {
        Ok(info) => println!("Address info: {:#?}", info),
        Err(e) => {
            eprintln!("{:?}", e);
        }
    }

    /*
    let boc = "";
    match api_client.send_message(boc).await {
        Ok(info) => println!("message hash: {:#?}", info.message_hash),
        Err(e) => {
            eprintln!("{:?}", e);
        }
    }
     */
}
