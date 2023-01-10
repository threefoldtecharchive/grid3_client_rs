use crate::runtimes::{devnet, mainnet, testnet, types};
use sp_core::crypto::SecretStringError;
use sp_core::{crypto::AccountId32, ed25519, sr25519};
use std::str::FromStr;
use subxt::{
    tx::{PairSigner, Signer},
    Error, OnlineClient, PolkadotConfig,
};
pub use types::{BlockNumber, Contract, Hash, SystemAccountInfo, TfgridFarm, TfgridNode, Twin};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Runtime {
    Devnet,
    Testnet,
    Mainnet,
}

impl FromStr for Runtime {
    type Err = &'static str;

    fn from_str(v: &str) -> Result<Self, Self::Err> {
        match v {
            "devnet" => Ok(Self::Devnet),
            "mainnet" => Ok(Self::Mainnet),
            "testnet" => Ok(Self::Testnet),
            _ => Err("unknown runtime"),
        }
    }
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum KeyType {
    Sr25519,
    Ed25519,
}

impl FromStr for KeyType {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "sr25519" => Ok(Self::Sr25519),
            "ed25519" => Ok(Self::Ed25519),
            _ => Err("unknown key type"),
        }
    }
}

pub enum Pair {
    Sr25519(sr25519::Pair),
    Ed25519(ed25519::Pair),
}

impl Pair {
    pub fn from_seed<S: AsRef<str>>(
        k: KeyType,
        seed: S,
        password: Option<&str>,
    ) -> Result<Self, SecretStringError> {
        let seed = seed
            .as_ref()
            .strip_prefix("0x")
            .unwrap_or_else(|| seed.as_ref());

        let pair = match k {
            KeyType::Sr25519 => {
                let (pair, _): (sr25519::Pair, _) =
                    sp_core::Pair::from_string_with_seed(seed, password)?;
                Self::Sr25519(pair)
            }
            KeyType::Ed25519 => {
                let (pair, _): (ed25519::Pair, _) =
                    sp_core::Pair::from_string_with_seed(seed, password)?;
                Self::Ed25519(pair)
            }
        };

        Ok(pair)
    }

    pub fn from_phrase<S: AsRef<str>>(
        k: KeyType,
        phrase: S,
        password: Option<&str>,
    ) -> Result<Self, SecretStringError> {
        let phrase = phrase.as_ref();

        let pair = match k {
            KeyType::Sr25519 => {
                let (pair, _): (sr25519::Pair, _) = sp_core::Pair::from_phrase(phrase, password)?;
                Self::Sr25519(pair)
            }
            KeyType::Ed25519 => {
                let (pair, _): (ed25519::Pair, _) = sp_core::Pair::from_phrase(phrase, password)?;
                Self::Ed25519(pair)
            }
        };

        Ok(pair)
    }

    pub fn signer(&self) -> Box<dyn Signer<PolkadotConfig> + Send + Sync> {
        match self {
            Self::Ed25519(pair) => Box::new(PairSigner::new(pair.clone())),
            Self::Sr25519(pair) => Box::new(PairSigner::new(pair.clone())),
        }
    }
}

impl From<sr25519::Pair> for Pair {
    fn from(value: sr25519::Pair) -> Self {
        Self::Sr25519(value)
    }
}

impl From<ed25519::Pair> for Pair {
    fn from(value: ed25519::Pair) -> Self {
        Self::Ed25519(value)
    }
}

pub struct Client {
    pub runtime: Runtime,
    pub pair: Pair,
    pub api: OnlineClient<PolkadotConfig>,
}

impl Client {
    pub async fn new(url: String, pair: Pair, runtime: Runtime) -> Result<Client, Error> {
        let api = OnlineClient::<PolkadotConfig>::from_url(url).await?;

        Ok(Client { pair, api, runtime })
    }

    pub async fn get_twin_by_id(
        &self,
        id: u32,
        at_block: Option<Hash>,
    ) -> Result<Option<Twin>, Error> {
        match self.runtime {
            Runtime::Devnet => devnet::get_twin_by_id(self, id, at_block).await,
            Runtime::Testnet => testnet::get_twin_by_id(self, id, at_block).await,
            Runtime::Mainnet => mainnet::get_twin_by_id(self, id, at_block).await,
        }
    }

    pub async fn get_farm_by_id(
        &self,
        id: u32,
        at_block: Option<Hash>,
    ) -> Result<Option<TfgridFarm>, Error> {
        match self.runtime {
            Runtime::Devnet => devnet::get_farm_by_id(self, id, at_block).await,
            Runtime::Testnet => testnet::get_farm_by_id(self, id, at_block).await,
            Runtime::Mainnet => mainnet::get_farm_by_id(self, id, at_block).await,
        }
    }

    pub async fn get_node_by_id(
        &self,
        id: u32,
        at_block: Option<Hash>,
    ) -> Result<Option<TfgridNode>, Error> {
        match self.runtime {
            Runtime::Devnet => devnet::get_node_by_id(self, id, at_block).await,
            Runtime::Testnet => testnet::get_node_by_id(self, id, at_block).await,
            Runtime::Mainnet => mainnet::get_node_by_id(self, id, at_block).await,
        }
    }

    pub async fn get_balance(
        &self,
        account: &AccountId32,
        at_block: Option<Hash>,
    ) -> Result<Option<SystemAccountInfo>, Error> {
        match self.runtime {
            Runtime::Devnet => devnet::get_balance(self, account, at_block).await,
            Runtime::Testnet => testnet::get_balance(self, account, at_block).await,
            Runtime::Mainnet => mainnet::get_balance(self, account, at_block).await,
        }
    }

    pub async fn get_block_hash(
        &self,
        block_number: Option<BlockNumber>,
    ) -> Result<Option<Hash>, Error> {
        match self.runtime {
            Runtime::Devnet => devnet::get_block_hash(self, block_number).await,
            Runtime::Testnet => testnet::get_block_hash(self, block_number).await,
            Runtime::Mainnet => mainnet::get_block_hash(self, block_number).await,
        }
    }

    pub async fn get_contract_by_id(
        &self,
        id: u64,
        at_block: Option<Hash>,
    ) -> Result<Option<Contract>, Error> {
        match self.runtime {
            Runtime::Devnet => devnet::get_contract_by_id(self, id, at_block).await,
            Runtime::Testnet => devnet::get_contract_by_id(self, id, at_block).await,
            Runtime::Mainnet => devnet::get_contract_by_id(self, id, at_block).await,
        }
    }
}
