use super::devnet::devnet::runtime_types::pallet_tfgrid::types::EntityProof as DevnetEntityProof;
use super::devnet::Twin as DevnetTwin;
use super::mainnet::mainnet::runtime_types::pallet_tfgrid::types::EntityProof as MainnetEntityProof;
use super::mainnet::Twin as MainnetTwin;
use serde::{Deserialize, Serialize};
use sp_core::crypto::AccountId32;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Twin {
    version: u32,
    id: u32,
    account_id: AccountId32,
    ip: String,
    entities: Vec<EntityProof>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct EntityProof {
    entity_id: u32,
    signature: String,
}

impl From<DevnetTwin> for Twin {
    fn from(twin: DevnetTwin) -> Self {
        let ip = String::from_utf8(twin.ip.0).expect("Found invalid UTF-8");
        let entities = twin.entities.into_iter().map(|e| e.into()).collect();

        Twin {
            version: twin.version,
            id: twin.id,
            account_id: twin.account_id,
            ip,
            entities,
        }
    }
}

impl From<DevnetEntityProof> for EntityProof {
    fn from(proof: DevnetEntityProof) -> Self {
        let signature = String::from_utf8(proof.signature).expect("Found invalid UTF-8");
        EntityProof {
            entity_id: proof.entity_id,
            signature,
        }
    }
}

impl From<MainnetTwin> for Twin {
    fn from(twin: MainnetTwin) -> Self {
        let ip = String::from_utf8(twin.ip.0).expect("Found invalid UTF-8");
        let entities = twin.entities.into_iter().map(|e| e.into()).collect();

        Twin {
            version: twin.version,
            id: twin.id,
            account_id: twin.account_id,
            ip,
            entities,
        }
    }
}

impl From<MainnetEntityProof> for EntityProof {
    fn from(proof: MainnetEntityProof) -> Self {
        let signature = String::from_utf8(proof.signature).expect("Found invalid UTF-8");
        EntityProof {
            entity_id: proof.entity_id,
            signature,
        }
    }
}