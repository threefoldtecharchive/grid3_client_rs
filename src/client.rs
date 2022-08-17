use regex::Regex;
use sp_core::{sr25519, Pair, H256};
use subxt::{tx::PairSigner, Error, OnlineClient, PolkadotConfig};
#[subxt::subxt(runtime_metadata_path = "artifacts/devnet.scale")]
pub mod devnet {
    #[subxt(substitute_type = "frame_support::storage::bounded_vec::BoundedVec")]
    use ::sp_std::vec::Vec;
}
use devnet::runtime_types::pallet_smart_contract::types::Contract;
use devnet::runtime_types::pallet_tfgrid::interface::{InterfaceIp, InterfaceMac, InterfaceName};
use devnet::runtime_types::pallet_tfgrid::pub_config::{Domain, GW4, GW6, IP4, IP6};
use devnet::runtime_types::pallet_tfgrid::twin::TwinIp;
use devnet::runtime_types::pallet_tfgrid::types::Twin as TwinData;
use devnet::runtime_types::tfchain_support::types::{
    Interface, Node as NodeData, PublicConfig, IP,
};

pub type Twin = TwinData<TwinIp, sp_core::crypto::AccountId32>;

pub type IPv4 = IP<IP4, GW4>;
pub type IPv6 = IP<IP6, GW6>;
pub type PublicConfigOf = PublicConfig<IPv4, Option<IPv6>, Option<Domain>>;
pub type InterfaceOf = Interface<InterfaceName, InterfaceMac, Vec<InterfaceIp>>;
pub type Node = NodeData<PublicConfigOf, InterfaceOf>;

pub struct TfchainClient {
    pub pair: sr25519::Pair,
    pub api: OnlineClient<PolkadotConfig>,
}

impl TfchainClient {
    pub async fn new(url: String, pair: sr25519::Pair) -> Result<TfchainClient, Error> {
        let api = OnlineClient::<PolkadotConfig>::from_url(url).await?;

        Ok(TfchainClient { pair, api })
    }

    pub async fn create_twin(&self, ip: String) -> Result<H256, Error> {
        let create_twin_tx = devnet::tx()
            .tfgrid_module()
            .create_twin(ip.as_bytes().to_vec());
        let signer = PairSigner::new(self.pair.clone());
        self.api
            .tx()
            .sign_and_submit_default(&create_twin_tx, &signer)
            .await
    }

    pub async fn sign_terms_and_conditions(
        &self,
        document_link: String,
        document_hash: String,
    ) -> Result<H256, Error> {
        let create_twin_tx = devnet::tx().tfgrid_module().user_accept_tc(
            document_link.as_bytes().to_vec(),
            document_hash.as_bytes().to_vec(),
        );
        let signer = PairSigner::new(self.pair.clone());
        self.api
            .tx()
            .sign_and_submit_default(&create_twin_tx, &signer)
            .await
    }

    pub async fn get_twin_by_id(&self, id: u32) -> Result<Twin, Error> {
        let twin = devnet::storage().tfgrid_module().twins(id);
        let t = self.api.storage().fetch(&twin, None).await?.unwrap();
        println!("twin: {:?}", t);

        Ok(t)
    }

    pub async fn get_contract_by_id(&self, id: u64) -> Result<Contract, Error> {
        let contract = devnet::storage().smart_contract_module().contracts(id);
        let c = self.api.storage().fetch(&contract, None).await?.unwrap();

        println!("contract: {:?}", c);

        Ok(c)
    }

    pub async fn get_node_by_id(&self, id: u32) -> Result<Node, Error> {
        let node = devnet::storage().tfgrid_module().nodes(id);
        let n = self.api.storage().fetch(&node, None).await?.unwrap();

        Ok(n)
    }
}

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed(seed: &str, pass: Option<&str>) -> sr25519::Pair {
    // Use regex to remove control characters
    let re = Regex::new(r"[\x00-\x1F]").unwrap();
    let clean_seed = re.replace_all(&seed.trim(), "");
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
