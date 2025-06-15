# LayerZero Scan Client

> The project has been unmaintained for several years now, and I think API has changed since then.

## Installation
```toml
layerzero_scan_client = "0.1"
```
## Usage
Initialize client with the desired environment
```rust
#[tokio::main]
async fn main() {
    let client = Client::new(Environment::Mainnet, None);
    let src_tx_hash = "0x...";
    let response = client.get_message_by_src_tx_hash(src_tx_hash).await.unwrap();
}
```
## Response
For new transactions the message list may be empty. Polling should be implemented at the application level since it is app specific. Expect the messages list to be empty for recent transactions, before moving to status: INFLIGHT, and finally status: DELIVERED.
Response will include `Vec<Message>` with following parameters:  
```rust
pub struct Message {
    pub src_ua_address: String,
    pub dst_ua_address: String,
    pub src_chain_id: u64,
    pub dst_chain_id: u64,
    pub dst_tx_hash: Option<String>,
    pub dst_tx_error: Option<String>,
    pub src_tx_hash: String,
    pub src_block_hash: String,
    pub src_block_number: String,
    pub src_ua_nonce: u64,
    pub status: MessageStatus,
}

pub enum MessageStatus {
    Inflight,
    Delivered,
    Failed,
}
```
### Note
This library is based on official [LayerZero JavaScript Client](https://www.npmjs.com/package/@layerzerolabs/scan-client?activeTab=readme)

