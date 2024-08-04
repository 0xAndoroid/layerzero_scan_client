#[cfg(test)]
mod test;

pub async fn get_message_by_src_tx_hash(
    src_chain_id: u64,
    src_tx_hash: &str,
) -> Result<GetMessageBySrcTxHashResponse, Error> {
    let env = if src_chain_id < 10000 {
        Environment::Mainnet
    } else if src_chain_id < 20000 {
        Environment::Testnet
    } else {
        Environment::Sandbox
    };

    let client = Client::new(env, None);
    client.get_message_by_src_tx_hash(src_tx_hash).await
}

pub struct Client {
    #[allow(dead_code)]
    options: ClientOptions,
    env: Environment,
}

impl Client {
    pub fn new(env: Environment, options: Option<ClientOptions>) -> Self {
        Client {
            options: options.unwrap_or_default(),
            env,
        }
    }

    pub async fn get_message_by_src_tx_hash<'a>(
        &self,
        src_tx_hash: &'a str,
    ) -> Result<GetMessageBySrcTxHashResponse, Error> {
        let url = match self.env {
            Environment::Testnet => TESTNET_URL,
            Environment::Mainnet => MAINNET_URL,
            Environment::Sandbox => SANDBOX_URL,
        };
        let path = format!("/tx/{}", src_tx_hash);
        let full_url = format!("{}{}", url, path);

        Ok(reqwest::get(full_url).await?.json::<GetMessageBySrcTxHashResponse>().await?)
    }
}

const TESTNET_URL: &str = "https://api-testnet.layerzero-scan.com";
const MAINNET_URL: &str = "https://api-mainnet.layerzero-scan.com";
const SANDBOX_URL: &str = "https://api-sandbox.layerzero-scan.com";

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
}

#[derive(Debug, Clone, serde::Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
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

#[derive(Debug, Clone, serde::Deserialize, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum MessageStatus {
    Inflight,
    Delivered,
    Failed,
}

#[derive(Debug, Clone, serde::Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetMessageBySrcTxHashResponse {
    pub messages: Vec<Message>,
}

#[derive(Default)]
pub struct ClientOptions {}

pub enum Environment {
    Testnet,
    Mainnet,
    Sandbox,
}
