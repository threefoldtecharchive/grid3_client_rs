#[subxt::subxt(runtime_metadata_path = "artifacts/testnet.scale")]
pub mod testnet {
    #[subxt(substitute_type = "frame_support::storage::bounded_vec::BoundedVec")]
    use ::sp_std::vec::Vec;
}
use super::types;
use sp_core::{crypto::AccountId32, H256};
use subxt::{tx::PairSigner, Error};
pub use testnet::runtime_types::frame_system::AccountInfo;
pub use testnet::runtime_types::pallet_balances::AccountData;
pub use testnet::runtime_types::pallet_smart_contract::types::Contract;
pub use testnet::runtime_types::pallet_tfgrid::{
    farm::FarmName,
    interface::{InterfaceIp, InterfaceMac, InterfaceName},
    pub_config::{Domain, GW4, GW6, IP4, IP6},
    pub_ip::{GatewayIP, PublicIP},
    twin::TwinIp,
    types::Twin as TwinData,
};
pub use testnet::runtime_types::tfchain_support::types::{
    Farm as FarmData, Interface, Node as NodeData, PublicConfig, PublicIP as PublicIpData, IP,
};

pub type Twin = TwinData<TwinIp, sp_core::crypto::AccountId32>;

pub type PublicIpOf = PublicIpData<PublicIP, GatewayIP>;
pub type Farm = FarmData<FarmName, PublicIpOf>;

pub type IPv4 = IP<IP4, GW4>;
pub type IPv6 = IP<IP6, GW6>;
pub type PublicConfigOf = PublicConfig<IPv4, Option<IPv6>, Option<Domain>>;
pub type InterfaceOf = Interface<InterfaceName, InterfaceMac, Vec<InterfaceIp>>;
pub type Node = NodeData<PublicConfigOf, InterfaceOf>;

use crate::client::TfchainClient;

pub use testnet::tft_bridge_module::events::BurnTransactionReady;
pub use testnet::tft_bridge_module::events::BurnTransactionSignatureAdded;
pub use testnet::tft_bridge_module::events::MintTransactionProposed;

pub type SystemAccountInfo = AccountInfo<u32, AccountData<u128>>;

pub async fn create_twin(cl: &TfchainClient, ip: String) -> Result<H256, Error> {
    let create_twin_tx = testnet::tx()
        .tfgrid_module()
        .create_twin(ip.as_bytes().to_vec());
    let signer = PairSigner::new(cl.pair.clone());
    cl.api
        .tx()
        .sign_and_submit_default(&create_twin_tx, &signer)
        .await
}

pub async fn sign_terms_and_conditions(
    cl: &TfchainClient,
    document_link: String,
    document_hash: String,
) -> Result<H256, Error> {
    let create_twin_tx = testnet::tx().tfgrid_module().user_accept_tc(
        document_link.as_bytes().to_vec(),
        document_hash.as_bytes().to_vec(),
    );
    let signer = PairSigner::new(cl.pair.clone());
    cl.api
        .tx()
        .sign_and_submit_default(&create_twin_tx, &signer)
        .await
}

pub async fn get_twin_by_id(
    cl: &TfchainClient,
    id: u32,
    at_block: Option<types::Hash>,
) -> Result<Option<types::Twin>, Error> {
    Ok(cl
        .api
        .storage()
        .fetch(&testnet::storage().tfgrid_module().twins(id), at_block)
        .await?
        .map(types::Twin::from))
}

pub async fn get_contract_by_id(cl: &TfchainClient, id: u64) -> Result<Option<Contract>, Error> {
    cl.api
        .storage()
        .fetch(
            &testnet::storage().smart_contract_module().contracts(id),
            None,
        )
        .await
}

pub async fn get_node_by_id(cl: &TfchainClient, id: u32) -> Result<Option<Node>, Error> {
    cl.api
        .storage()
        .fetch(&testnet::storage().tfgrid_module().nodes(id), None)
        .await
}

pub async fn get_farm_by_id(
    cl: &TfchainClient,
    id: u32,
    at_block: Option<types::Hash>,
) -> Result<Option<types::TfgridFarm>, Error> {
    Ok(cl
        .api
        .storage()
        .fetch(&testnet::storage().tfgrid_module().farms(id), at_block)
        .await?
        .map(types::TfgridFarm::from))
}

pub async fn get_block_hash(
    cl: &TfchainClient,
    block_number: Option<types::BlockNumber>,
) -> Result<Option<types::Hash>, Error> {
    cl.api.rpc().block_hash(block_number).await
}

pub async fn get_balance(
    cl: &TfchainClient,
    account: &AccountId32,
    at_block: Option<types::Hash>,
) -> Result<Option<types::SystemAccountInfo>, Error> {
    Ok(cl
        .api
        .storage()
        .fetch(&testnet::storage().system().account(account), at_block)
        .await?
        .map(types::SystemAccountInfo::from))
}
