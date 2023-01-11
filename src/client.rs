use crate::runtimes::{devnet, local, mainnet, testnet, types};
use std::str::FromStr;
use subxt::{
    ext::{
        sp_core::{crypto::SecretStringError, ed25519, sr25519, Pair},
        sp_runtime::AccountId32,
    },
    tx::{PairSigner, Signer},
    Error, OnlineClient, PolkadotConfig,
};
pub use types::{BlockNumber, Contract, Hash, SystemAccountInfo, TfgridFarm, TfgridNode, Twin};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Runtime {
    Local,
    Devnet,
    Testnet,
    Mainnet,
}

impl FromStr for Runtime {
    type Err = &'static str;

    fn from_str(v: &str) -> Result<Self, Self::Err> {
        match v {
            "local" => Ok(Self::Local),
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

pub enum KeyPair {
    Sr25519(sr25519::Pair),
    Ed25519(ed25519::Pair),
}

impl KeyPair {
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
                let (pair, _): (sr25519::Pair, _) = Pair::from_string_with_seed(seed, password)?;
                Self::Sr25519(pair)
            }
            KeyType::Ed25519 => {
                let (pair, _): (ed25519::Pair, _) = Pair::from_string_with_seed(seed, password)?;
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
                let (pair, _): (sr25519::Pair, _) = Pair::from_phrase(phrase, password)?;
                Self::Sr25519(pair)
            }
            KeyType::Ed25519 => {
                let (pair, _): (ed25519::Pair, _) = Pair::from_phrase(phrase, password)?;
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

impl From<sr25519::Pair> for KeyPair {
    fn from(value: sr25519::Pair) -> Self {
        Self::Sr25519(value)
    }
}

impl From<ed25519::Pair> for KeyPair {
    fn from(value: ed25519::Pair) -> Self {
        Self::Ed25519(value)
    }
}

pub struct Client {
    pub runtime: Runtime,
    pub pair: KeyPair,
    pub api: OnlineClient<PolkadotConfig>,
}

macro_rules! call {
    ($self:ident, $name:ident, $($arg:expr),+) => (
        match $self.runtime {
            Runtime::Local => local::$name($self, $($arg),+).await,
            Runtime::Devnet => devnet::$name($self, $($arg),+).await,
            Runtime::Testnet => testnet::$name($self, $($arg),+).await,
            Runtime::Mainnet => mainnet::$name($self, $($arg),+).await,
        }
    )
}

impl Client {
    pub async fn new(url: String, pair: KeyPair, runtime: Runtime) -> Result<Client, Error> {
        let api = OnlineClient::<PolkadotConfig>::from_url(url).await?;

        Ok(Client { pair, api, runtime })
    }

    pub async fn create_twin(
        &self,
        relay: Option<String>,
        pk: Option<String>,
    ) -> Result<Hash, Error> {
        call!(self, create_twin, relay, pk)
    }

    pub async fn update_twin(
        &self,
        relay: Option<String>,
        pk: Option<String>,
    ) -> Result<Hash, Error> {
        call!(self, update_twin, relay, pk)
    }

    pub async fn sign_terms_and_conditions(
        &self,
        document_link: String,
        document_hash: String,
    ) -> Result<Hash, Error> {
        call!(
            self,
            sign_terms_and_conditions,
            document_link,
            document_hash
        )
    }

    pub async fn get_twin_by_id(
        &self,
        id: u32,
        at_block: Option<Hash>,
    ) -> Result<Option<Twin>, Error> {
        call!(self, get_twin_by_id, id, at_block)
    }

    pub async fn get_twin_id_by_account(
        &self,
        account: AccountId32,
        at_block: Option<types::Hash>,
    ) -> Result<Option<u32>, Error> {
        call!(self, get_twin_id_by_account, account, at_block)
    }

    pub async fn get_farm_by_id(
        &self,
        id: u32,
        at_block: Option<Hash>,
    ) -> Result<Option<TfgridFarm>, Error> {
        call!(self, get_farm_by_id, id, at_block)
    }

    pub async fn get_node_by_id(
        &self,
        id: u32,
        at_block: Option<Hash>,
    ) -> Result<Option<TfgridNode>, Error> {
        call!(self, get_node_by_id, id, at_block)
    }

    pub async fn get_balance(
        &self,
        account: &AccountId32,
        at_block: Option<Hash>,
    ) -> Result<Option<SystemAccountInfo>, Error> {
        call!(self, get_balance, account, at_block)
    }

    pub async fn get_block_hash(
        &self,
        block_number: Option<BlockNumber>,
    ) -> Result<Option<Hash>, Error> {
        call!(self, get_block_hash, block_number)
    }

    pub async fn get_contract_by_id(
        &self,
        id: u64,
        at_block: Option<Hash>,
    ) -> Result<Option<Contract>, Error> {
        call!(self, get_contract_by_id, id, at_block)
    }
}
