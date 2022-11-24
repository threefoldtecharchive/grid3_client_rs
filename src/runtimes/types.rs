use serde::{Deserialize, Serialize};
use sp_core::crypto::AccountId32;
use subxt::{Config, PolkadotConfig};

use frame_system::AccountInfo;
use pallet_balances::AccountData;

use tfchain_support::types::FarmCertification as SupportFarmCertification;

use super::devnet::devnet::runtime_types::pallet_tfgrid::types::EntityProof as DevnetEntityProof;
use super::devnet::devnet::runtime_types::tfchain_support::types::FarmCertification as DevnetFarmCertification;
use super::devnet::{
    Farm as DevnetFarm, SystemAccountInfo as DevnetSystemAccountInfo, Twin as DevnetTwin,
};

use super::mainnet::mainnet::runtime_types::pallet_tfgrid::types::EntityProof as MainnetEntityProof;
use super::mainnet::mainnet::runtime_types::tfchain_support::types::FarmCertification as MainnetFarmCertification;
use super::mainnet::{
    Farm as MainnetFarm, SystemAccountInfo as MainnetSystemAccountInfo, Twin as MainnetTwin,
};

use super::testnet::testnet::runtime_types::pallet_tfgrid::types::EntityProof as TestnetEntityProof;
use super::testnet::testnet::runtime_types::tfchain_support::types::FarmCertification as TestnetFarmCertification;
use super::testnet::{
    Farm as TestnetFarm, SystemAccountInfo as TestnetSystemAccountInfo, Twin as TestnetTwin,
};

pub type Hash = <PolkadotConfig as Config>::Hash;
pub type BlockNumber = subxt::rpc::BlockNumber;

#[derive(Debug, Clone)]

pub struct TfgridFarm {
    pub version: u32,
    pub id: u32,
    pub name: String,
    pub twin_id: u32,
    pub pricing_policy_id: u32,
    pub certification: SupportFarmCertification,
    pub dedicated_farm: bool,
    pub farming_policy_limits: Option<FarmingPolicyLimit>,
    pub public_ips: Vec<FarmPublicIP>,
}

#[derive(Debug, Clone)]

pub struct FarmPublicIP {
    pub ip: String,
    pub gateway: String,
    pub contract_id: u64,
}

#[derive(Debug, Clone)]

pub struct FarmingPolicyLimit {
    pub farming_policy_id: u32,
    pub cu: Option<u64>,
    pub su: Option<u64>,
    pub end: Option<u64>,
    pub node_count: Option<u32>,
    pub node_certification: bool,
}

impl From<MainnetFarm> for TfgridFarm {
    fn from(farm: MainnetFarm) -> Self {
        let farm_name = String::from_utf8(farm.name.0).expect("Found invalid UTF-8");

        let limit: Option<FarmingPolicyLimit> = match farm.farming_policy_limits {
            Some(lim) => Some(FarmingPolicyLimit {
                cu: lim.cu,
                su: lim.su,
                end: lim.end,
                farming_policy_id: lim.farming_policy_id,
                node_certification: lim.node_certification,
                node_count: lim.node_count,
            }),
            None => None,
        };

        let mut public_ips = vec![];
        for ip in farm.public_ips {
            public_ips.push(FarmPublicIP {
                ip: String::from_utf8(ip.ip.0).expect("Found invalid UTF-8"),
                gateway: String::from_utf8(ip.gateway.0).expect("Found invalid UTF-8"),
                contract_id: ip.contract_id,
            })
        }

        let farm_certification: SupportFarmCertification = match farm.certification {
            MainnetFarmCertification::Gold => SupportFarmCertification::Gold,
            MainnetFarmCertification::NotCertified => SupportFarmCertification::NotCertified,
        };

        TfgridFarm {
            version: farm.version,
            id: farm.id,
            name: farm_name,
            twin_id: farm.twin_id,
            pricing_policy_id: farm.pricing_policy_id,
            certification: farm_certification,
            dedicated_farm: farm.dedicated_farm,
            farming_policy_limits: limit,
            public_ips,
        }
    }
}

impl From<TestnetFarm> for TfgridFarm {
    fn from(farm: TestnetFarm) -> Self {
        let farm_name = String::from_utf8(farm.name.0).expect("Found invalid UTF-8");

        let limit: Option<FarmingPolicyLimit> = match farm.farming_policy_limits {
            Some(lim) => Some(FarmingPolicyLimit {
                cu: lim.cu,
                su: lim.su,
                end: lim.end,
                farming_policy_id: lim.farming_policy_id,
                node_certification: lim.node_certification,
                node_count: lim.node_count,
            }),
            None => None,
        };

        let mut public_ips = vec![];
        for ip in farm.public_ips {
            public_ips.push(FarmPublicIP {
                ip: String::from_utf8(ip.ip.0).expect("Found invalid UTF-8"),
                gateway: String::from_utf8(ip.gateway.0).expect("Found invalid UTF-8"),
                contract_id: ip.contract_id,
            })
        }

        let farm_certification: SupportFarmCertification = match farm.certification {
            TestnetFarmCertification::Gold => SupportFarmCertification::Gold,
            TestnetFarmCertification::NotCertified => SupportFarmCertification::NotCertified,
        };

        TfgridFarm {
            version: farm.version,
            id: farm.id,
            name: farm_name,
            twin_id: farm.twin_id,
            pricing_policy_id: farm.pricing_policy_id,
            certification: farm_certification,
            dedicated_farm: farm.dedicated_farm,
            farming_policy_limits: limit,
            public_ips,
        }
    }
}

impl From<DevnetFarm> for TfgridFarm {
    fn from(farm: DevnetFarm) -> Self {
        let farm_name = String::from_utf8(farm.name.0).expect("Found invalid UTF-8");

        let limit: Option<FarmingPolicyLimit> = match farm.farming_policy_limits {
            Some(lim) => Some(FarmingPolicyLimit {
                cu: lim.cu,
                su: lim.su,
                end: lim.end,
                farming_policy_id: lim.farming_policy_id,
                node_certification: lim.node_certification,
                node_count: lim.node_count,
            }),
            None => None,
        };

        let mut public_ips = vec![];
        for ip in farm.public_ips {
            public_ips.push(FarmPublicIP {
                ip: String::from_utf8(ip.ip.0).expect("Found invalid UTF-8"),
                gateway: String::from_utf8(ip.gateway.0).expect("Found invalid UTF-8"),
                contract_id: ip.contract_id,
            })
        }

        let farm_certification: SupportFarmCertification = match farm.certification {
            DevnetFarmCertification::Gold => SupportFarmCertification::Gold,
            DevnetFarmCertification::NotCertified => SupportFarmCertification::NotCertified,
        };

        TfgridFarm {
            version: farm.version,
            id: farm.id,
            name: farm_name,
            twin_id: farm.twin_id,
            pricing_policy_id: farm.pricing_policy_id,
            certification: farm_certification,
            dedicated_farm: farm.dedicated_farm,
            farming_policy_limits: limit,
            public_ips,
        }
    }
}

pub type SystemAccountInfo = AccountInfo<u32, AccountData<u128>>;

impl From<MainnetSystemAccountInfo> for SystemAccountInfo {
    fn from(info: MainnetSystemAccountInfo) -> Self {
        SystemAccountInfo {
            nonce: info.nonce,
            consumers: info.consumers,
            providers: info.providers,
            sufficients: info.sufficients,
            data: pallet_balances::AccountData {
                free: info.data.free,
                fee_frozen: info.data.fee_frozen,
                misc_frozen: info.data.misc_frozen,
                reserved: info.data.reserved,
            },
        }
    }
}

impl From<TestnetSystemAccountInfo> for SystemAccountInfo {
    fn from(info: TestnetSystemAccountInfo) -> Self {
        SystemAccountInfo {
            nonce: info.nonce,
            consumers: info.consumers,
            providers: info.providers,
            sufficients: info.sufficients,
            data: pallet_balances::AccountData {
                free: info.data.free,
                fee_frozen: info.data.fee_frozen,
                misc_frozen: info.data.misc_frozen,
                reserved: info.data.reserved,
            },
        }
    }
}

impl From<DevnetSystemAccountInfo> for SystemAccountInfo {
    fn from(info: DevnetSystemAccountInfo) -> Self {
        SystemAccountInfo {
            nonce: info.nonce,
            consumers: info.consumers,
            providers: info.providers,
            sufficients: info.sufficients,
            data: pallet_balances::AccountData {
                free: info.data.free,
                fee_frozen: info.data.fee_frozen,
                misc_frozen: info.data.misc_frozen,
                reserved: info.data.reserved,
            },
        }
    }
}

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

impl From<TestnetTwin> for Twin {
    fn from(twin: TestnetTwin) -> Self {
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

impl From<TestnetEntityProof> for EntityProof {
    fn from(proof: TestnetEntityProof) -> Self {
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
