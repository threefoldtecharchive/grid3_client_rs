use crate::runtimes::{devnet, mainnet, support::SupportedRuntime, testnet, types};
use regex::Regex;
use sp_core::{crypto::AccountId32, sr25519, Pair};
use std::str::FromStr;
use subxt::{Error, OnlineClient, PolkadotConfig};
pub use types::{BlockNumber, Hash, SystemAccountInfo, TfgridFarm, TfgridNode, Twin};

pub struct TfchainClient {
    pub runtime: SupportedRuntime,
    pub pair: sr25519::Pair,
    pub api: OnlineClient<PolkadotConfig>,
}

impl TfchainClient {
    pub async fn new(
        url: String,
        pair: sr25519::Pair,
        network: &str,
    ) -> Result<TfchainClient, Error> {
        let api = OnlineClient::<PolkadotConfig>::from_url(url).await?;

        let runtime = SupportedRuntime::from_str(network).expect("runtime not supported");

        Ok(TfchainClient { pair, api, runtime })
    }

    pub async fn get_twin_by_id(
        &self,
        id: u32,
        at_block: Option<Hash>,
    ) -> Result<Option<Twin>, Error> {
        match self.runtime {
            SupportedRuntime::Devnet => devnet::get_twin_by_id(self, id, at_block).await,
            SupportedRuntime::Testnet => testnet::get_twin_by_id(self, id, at_block).await,
            SupportedRuntime::Mainnet => mainnet::get_twin_by_id(self, id, at_block).await,
        }
    }

    pub async fn get_farm_by_id(
        &self,
        id: u32,
        at_block: Option<Hash>,
    ) -> Result<Option<TfgridFarm>, Error> {
        match self.runtime {
            SupportedRuntime::Devnet => devnet::get_farm_by_id(self, id, at_block).await,
            SupportedRuntime::Testnet => testnet::get_farm_by_id(self, id, at_block).await,
            SupportedRuntime::Mainnet => mainnet::get_farm_by_id(self, id, at_block).await,
        }
    }

    pub async fn get_node_by_id(
        &self,
        id: u32,
        at_block: Option<Hash>,
    ) -> Result<Option<TfgridNode>, Error> {
        match self.runtime {
            SupportedRuntime::Devnet => devnet::get_node_by_id(self, id, at_block).await,
            SupportedRuntime::Testnet => testnet::get_node_by_id(self, id, at_block).await,
            SupportedRuntime::Mainnet => mainnet::get_node_by_id(self, id, at_block).await,
        }
    }

    pub async fn get_balance(
        &self,
        account: &AccountId32,
        at_block: Option<Hash>,
    ) -> Result<Option<SystemAccountInfo>, Error> {
        match self.runtime {
            SupportedRuntime::Devnet => devnet::get_balance(self, account, at_block).await,
            SupportedRuntime::Testnet => testnet::get_balance(self, account, at_block).await,
            SupportedRuntime::Mainnet => mainnet::get_balance(self, account, at_block).await,
        }
    }

    pub async fn get_block_hash(
        &self,
        block_number: Option<BlockNumber>,
    ) -> Result<Option<Hash>, Error> {
        match self.runtime {
            SupportedRuntime::Devnet => devnet::get_block_hash(self, block_number).await,
            SupportedRuntime::Testnet => testnet::get_block_hash(self, block_number).await,
            SupportedRuntime::Mainnet => mainnet::get_block_hash(self, block_number).await,
        }
    }
}

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed(seed: &str, pass: Option<&str>) -> sr25519::Pair {
    // Use regex to remove control characters
    let re = Regex::new(r"[\x00-\x1F]").unwrap();
    let clean_seed = re.replace_all(seed.trim(), "");
    Pair::from_string_with_seed(&clean_seed, pass)
        .expect("constructed from known-good static value; qed")
        .0
}

/// Helper function to generate a crypto pair from seed
pub fn get_from_phrase(phrase: &str, pass: Option<&str>) -> sr25519::Pair {
    // Use regex to remove control characters
    Pair::from_phrase(phrase, pass)
        .expect("constructed from known-good static value; qed")
        .0
}
