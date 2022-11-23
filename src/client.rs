use crate::runtimes::{devnet, mainnet, support::SupportedRuntime, types};
use regex::Regex;
use sp_core::{crypto::AccountId32, sr25519, Pair};
use std::str::FromStr;
use subxt::{Config, Error, OnlineClient, PolkadotConfig};

// BlockHash
pub type Hash = <PolkadotConfig as Config>::Hash;

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

    pub async fn get_twin_by_id(&self, id: u32) -> Result<Option<types::Twin>, Error> {
        match self.runtime {
            SupportedRuntime::Devnet => devnet::get_twin_by_id(self, id).await,
            SupportedRuntime::Mainnet => mainnet::get_twin_by_id(self, id).await,
        }
    }

    pub async fn get_balance(
        &self,
        account: AccountId32,
    ) -> Result<Option<types::SystemAccountInfo>, Error> {
        match self.runtime {
            SupportedRuntime::Devnet => devnet::get_balance(self, account).await,
            SupportedRuntime::Mainnet => mainnet::get_balance(self, account).await,
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
