#[subxt::subxt(runtime_metadata_path = "artifacts/devnet.scale")]
pub mod devnet {
    #[subxt(substitute_type = "frame_support::storage::bounded_vec::BoundedVec")]
    use ::sp_std::vec::Vec;
    #[subxt(substitute_type = "bounded::bounded::BoundedVec")]
    use ::sp_std::vec::Vec;
}
use super::types;
pub use devnet::runtime_types::frame_system::AccountInfo;
pub use devnet::runtime_types::pallet_balances::AccountData;
pub use devnet::runtime_types::pallet_smart_contract::types::Contract;
pub use devnet::runtime_types::pallet_tfgrid::{
    farm::FarmName,
    interface::{InterfaceIp, InterfaceMac, InterfaceName},
    node::{Location, SerialNumber},
    types::Twin as TwinData,
};
pub use devnet::runtime_types::tfchain_support::types::{
    Farm as FarmData, Interface, Node as NodeData, PublicConfig, PublicIP as PublicIpData,
};
use subxt::ext::{sp_core::H256, sp_runtime::AccountId32};

use subxt::Error;

use devnet::runtime_types::sp_core::bounded::bounded_vec::BoundedVec;

pub type Twin = TwinData<AccountId32>;

pub type Farm = FarmData<FarmName>;

pub type InterfaceOf = Interface<InterfaceName, InterfaceMac, BoundedVec<InterfaceIp>>;
pub type Node = NodeData<Location, InterfaceOf, SerialNumber>;

pub type SystemAccountInfo = AccountInfo<u32, AccountData<u128>>;

use crate::client::Client;

pub use devnet::tft_bridge_module::events::BurnTransactionReady;
pub use devnet::tft_bridge_module::events::BurnTransactionSignatureAdded;
pub use devnet::tft_bridge_module::events::MintTransactionProposed;

pub async fn create_twin(
    cl: &Client,
    relay: Option<String>,
    pk: Option<String>,
) -> Result<H256, Error> {
    let r = match relay {
        Some(rel) => BoundedVec(rel.as_bytes().to_vec()),
        None => BoundedVec(Vec::new()),
    };

    let p = match pk {
        Some(pubk) => BoundedVec(pubk.as_bytes().to_vec()),
        None => BoundedVec(Vec::new()),
    };

    let create_twin_tx = devnet::tx().tfgrid_module().create_twin(r, p);
    let signer = cl.pair.signer();

    cl.api
        .tx()
        .sign_and_submit_default(&create_twin_tx, signer.as_ref())
        .await
}

pub async fn sign_terms_and_conditions(
    cl: &Client,
    document_link: String,
    document_hash: String,
) -> Result<H256, Error> {
    let create_twin_tx = devnet::tx().tfgrid_module().user_accept_tc(
        BoundedVec(document_link.as_bytes().to_vec()),
        BoundedVec(document_hash.as_bytes().to_vec()),
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
        .fetch(&devnet::storage().tfgrid_module().twins(id), at_block)
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
            &devnet::storage()
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
            &devnet::storage().smart_contract_module().contracts(id),
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
        .fetch(&devnet::storage().tfgrid_module().nodes(id), at_block)
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
        .fetch(&devnet::storage().tfgrid_module().farms(id), at_block)
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
        .fetch(&devnet::storage().system().account(account), at_block)
        .await?
        .map(types::SystemAccountInfo::from))
}
