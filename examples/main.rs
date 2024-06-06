use tokio;
use toncenter::client::{ApiClientV2, ApiKey, Network};

#[tokio::main]
async fn main() {
    let api_key = "a8b61ced4be11488cb6e82d65b93e3d4a29d20af406aed9688b9e0077e2dc742".to_string();
    let address = "0QCbOix87iy37AwRCWaYhJHzc2gXE_WnAG5vVEAySNT7zClz";

    let api_client = ApiClientV2::new(Network::Testnet, Some(ApiKey::Header(api_key)));

    match api_client.get_extended_address_information(address).await {
        Ok(info) => {
            println!("\nTestnet Extended Address Information:");
            println!("Type: {}", info.type_field);
            println!("Address: {}", info.address.account_address);
            println!("Balance: {}", info.balance);
            println!("Last Transaction ID: {:?}", info.last_transaction_id);
            println!("Block ID: {:?}", info.block_id);
            println!("Sync UTime: {}", info.sync_utime);
            println!("Account State: {:?}", info.account_state);
            println!("Revision: {}", info.revision);
        }
        Err(e) => eprintln!("{}", e),
    }
}
