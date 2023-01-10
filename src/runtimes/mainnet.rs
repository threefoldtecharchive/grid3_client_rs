#[subxt::subxt(runtime_metadata_path = "artifacts/mainnet.scale")]
pub mod mainnet {
    #[subxt(substitute_type = "frame_support::storage::bounded_vec::BoundedVec")]
    use ::sp_std::vec::Vec;
}
use super::types;
pub use mainnet::runtime_types::frame_system::AccountInfo;
pub use mainnet::runtime_types::pallet_balances::AccountData;
pub use mainnet::runtime_types::pallet_smart_contract::types::Contract;
pub use mainnet::runtime_types::pallet_tfgrid::{
    farm::FarmName,
    interface::{InterfaceIp, InterfaceMac, InterfaceName},
    pub_config::{Domain, GW4, GW6, IP4, IP6},
    pub_ip::{GatewayIP, PublicIP},
    twin::TwinIp,
    types::Twin as TwinData,
};
pub use mainnet::runtime_types::tfchain_support::types::{
    Farm as FarmData, Interface, Node as NodeData, PublicConfig, PublicIP as PublicIpData, IP,
};
use subxt::ext::{sp_core::H256, sp_runtime::AccountId32};

use subxt::Error;

pub type Twin = TwinData<TwinIp, AccountId32>;

pub type PublicIpOf = PublicIpData<PublicIP, GatewayIP>;
pub type Farm = FarmData<FarmName, PublicIpOf>;

pub type IPv4 = IP<IP4, GW4>;
pub type IPv6 = IP<IP6, GW6>;
pub type PublicConfigOf = PublicConfig<IPv4, Option<IPv6>, Option<Domain>>;
pub type InterfaceOf = Interface<InterfaceName, InterfaceMac, Vec<InterfaceIp>>;
pub type Node = NodeData<PublicConfigOf, InterfaceOf>;

use crate::client::Client;

pub use mainnet::tft_bridge_module::events::BurnTransactionReady;
pub use mainnet::tft_bridge_module::events::BurnTransactionSignatureAdded;
pub use mainnet::tft_bridge_module::events::MintTransactionProposed;

pub type SystemAccountInfo = AccountInfo<u32, AccountData<u128>>;

pub async fn create_twin(
    cl: &Client,
    ip: Option<String>,
    _pk: Option<String>,
) -> Result<H256, Error> {
    let create_twin_tx = mainnet::tx()
        .tfgrid_module()
        .create_twin(ip.unwrap().as_bytes().to_vec());
    let signer = cl.pair.signer();
    cl.api
        .tx()
        .sign_and_submit_default(&create_twin_tx, signer.as_ref())
        .await
}

pub async fn update_twin(
    cl: &Client,
    ip: Option<String>,
    _pk: Option<String>,
) -> Result<H256, Error> {
    let update_twin_tx = mainnet::tx()
        .tfgrid_module()
        .update_twin(ip.unwrap().as_bytes().to_vec());
    let signer = cl.pair.signer();
    cl.api
        .tx()
        .sign_and_submit_default(&update_twin_tx, signer.as_ref())
        .await
}

pub async fn sign_terms_and_conditions(
    cl: &Client,
    document_link: String,
    document_hash: String,
) -> Result<H256, Error> {
    let create_twin_tx = mainnet::tx().tfgrid_module().user_accept_tc(
        document_link.as_bytes().to_vec(),
        document_hash.as_bytes().to_vec(),
    );
    let signer = cl.pair.signer();
    cl.api
        .tx()
        .sign_and_submit_default(&create_twin_tx, signer.as_ref())
        .await
}

pub async fn get_twin_by_id(
    cl: &Client,
    id: u32,
    at_block: Option<types::Hash>,
) -> Result<Option<types::Twin>, Error> {
    Ok(cl
        .api
        .storage()
        .fetch(&mainnet::storage().tfgrid_module().twins(id), at_block)
        .await?
        .map(types::Twin::from))
}

pub async fn get_twin_id_by_account(
    cl: &Client,
    account: AccountId32,
    at_block: Option<types::Hash>,
) -> Result<Option<u32>, Error> {
    cl.api
        .storage()
        .fetch(
            &mainnet::storage()
                .tfgrid_module()
                .twin_id_by_account_id(account),
            at_block,
        )
        .await
}

pub async fn get_contract_by_id(
    cl: &Client,
    id: u64,
    at_block: Option<types::Hash>,
) -> Result<Option<types::Contract>, Error> {
    Ok(cl
        .api
        .storage()
        .fetch(
            &mainnet::storage().smart_contract_module().contracts(id),
            at_block,
        )
        .await?
        .map(types::Contract::from))
}

pub async fn get_node_by_id(
    cl: &Client,
    id: u32,
    at_block: Option<types::Hash>,
) -> Result<Option<types::TfgridNode>, Error> {
    Ok(cl
        .api
        .storage()
        .fetch(&mainnet::storage().tfgrid_module().nodes(id), at_block)
        .await?
        .map(types::TfgridNode::from))
}

pub async fn get_farm_by_id(
    cl: &Client,
    id: u32,
    at_block: Option<types::Hash>,
) -> Result<Option<types::TfgridFarm>, Error> {
    Ok(cl
        .api
        .storage()
        .fetch(&mainnet::storage().tfgrid_module().farms(id), at_block)
        .await?
        .map(types::TfgridFarm::from))
}

pub async fn get_block_hash(
    cl: &Client,
    block_number: Option<types::BlockNumber>,
) -> Result<Option<types::Hash>, Error> {
    cl.api.rpc().block_hash(block_number).await
}

pub async fn get_balance(
    cl: &Client,
    account: &AccountId32,
    at_block: Option<types::Hash>,
) -> Result<Option<types::SystemAccountInfo>, Error> {
    Ok(cl
        .api
        .storage()
        .fetch(&mainnet::storage().system().account(account), at_block)
        .await?
        .map(|t| types::SystemAccountInfo::from(t)))
}
