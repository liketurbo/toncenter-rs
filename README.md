# toncenter-rs

[![Latest version](https://img.shields.io/crates/v/toncenter.svg)](https://crates.io/crates/toncenter)

SDK for integrating Toncenter into Rust applications. Connect and interact with the Toncenter API effortlessly.

## Features

* **Authorization Support**: You can obtain token from [@tonapibot](https://t.me/tonapibot).
* **REST API v2 Integration**: Interact with Toncenter RESTful endpoints for API v2.
* **REST API v3 Integration**: Interact with Toncenter RESTful endpoints for API v3 (in progress).
* **JSON-RPC API Integration**: Utilize JSON-RPC protocol for all available methods.

## Installation

```toml
# Cargo.toml
[dependencies]
toncenter = "0.1.0"
```

## Usage

```rust
use toncenter::client::{ApiClientV2, ApiKey, Network};

#[tokio::main]
async fn main() {
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
}
```

## Contributing

Contributions to this library is welcomed! If you'd like to contribute, please feel free to open a pull request on GitHub.

## License

This project is licensed under the MIT License.

## Acknowledgments

Special thanks to the Toncenter team for providing a robust API to interact with the TON blockchain.
