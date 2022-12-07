use serde::{Deserialize, Serialize};
use sp_core::crypto::AccountId32;
use subxt::{Config, PolkadotConfig};

use frame_system::AccountInfo;
use pallet_balances::AccountData;

use tfchain_support::types::FarmCertification as SupportFarmCertification;

use super::devnet::devnet::runtime_types::pallet_tfgrid::types::EntityProof as DevnetEntityProof;
use super::devnet::devnet::runtime_types::tfchain_support::types::{FarmCertification as DevnetFarmCertification, NodeCertification as DevnetNodeCertification};
use super::devnet::{
    Farm as DevnetFarm, Node as DevnetNode, SystemAccountInfo as DevnetSystemAccountInfo,
    Twin as DevnetTwin,
};

use super::mainnet::mainnet::runtime_types::pallet_tfgrid::types::EntityProof as MainnetEntityProof;
use super::mainnet::mainnet::runtime_types::tfchain_support::types::{FarmCertification as MainnetFarmCertification, NodeCertification as MainnetNodeCertification};
use super::mainnet::{
    Farm as MainnetFarm, SystemAccountInfo as MainnetSystemAccountInfo, Twin as MainnetTwin, Node as MainnetNode,
};

use super::testnet::testnet::runtime_types::pallet_tfgrid::types::EntityProof as TestnetEntityProof;
use super::testnet::testnet::runtime_types::tfchain_support::types::{FarmCertification as TestnetFarmCertification, NodeCertification as TestnetNodeCertification};
use super::testnet::{
    Farm as TestnetFarm, SystemAccountInfo as TestnetSystemAccountInfo, Twin as TestnetTwin, Node as TestnetNode
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
pub struct TfgridNode {
    pub version: u32,
    pub id: u32,
    pub farm_id: u32,
    pub twin_id: u32,
    pub resources: ConsumableResources,
    pub location: Location,
    pub power: Power,
    pub public_config: Option<PublicConfig>,
    pub created: u64,
    pub farming_policy_id: u32,
    pub interfaces: Vec<Interface>,
    pub certification: NodeCertification,
    pub secure_boot: bool,
    pub serial_number: Option<String>,
    pub connection_price: u32,
}

#[derive(Debug, Clone, Default)]
pub struct ConsumableResources {
    pub total_resources: Resources,
    pub used_resources: Resources,
}

#[derive(Debug, Clone, Default)]
pub struct Resources {
    pub hru: u64,
    pub sru: u64,
    pub cru: u64,
    pub mru: u64,
}

#[derive(Debug, Clone)]
pub struct Location {
    pub city: String,
    pub country: String,
    pub latitude: String,
    pub longitude: String,
}

#[derive(Debug, Clone)]
pub struct IP {
    pub ip: String,
    pub gw: String,
}

#[derive(Debug, Clone)]
pub struct PublicConfig {
    pub ip4: IP,
    pub ip6: Option<IP>,
    pub domain: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Power {
    pub target: PowerTarget,
    pub state: PowerState,
    pub last_uptime: u64,
}

#[derive(Debug, Clone)]
pub enum PowerTarget {
    Up,
    Down,
}

#[derive(Debug, Clone)]
pub enum PowerState {
    Up,
    Down(u32),
}

#[derive(Debug, Clone)]
pub enum NodeCertification {
    Diy,
    Certified,
}

#[derive(Debug, Clone)]
pub struct Interface {
    pub name: String,
    pub mac: String,
    pub ips: Vec<String>,
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

impl From<DevnetNode> for TfgridNode {
    fn from(node: DevnetNode) -> Self {
        let mut resources = ConsumableResources::default();
        resources.total_resources.cru = node.resources.cru;
        resources.total_resources.hru = node.resources.hru;
        resources.total_resources.mru = node.resources.mru;
        resources.total_resources.sru = node.resources.sru;

        let location = Location {
            city: String::from_utf8(node.city).expect("valid city"),
            country: String::from_utf8(node.country).expect("valid city"),
            latitude: String::from_utf8(node.location.latitude).expect("valid city"),
            longitude: String::from_utf8(node.location.longitude).expect("valid city"),
        };

        let public_config = match node.public_config {
            Some(config) => {
                let mut pub_conf = PublicConfig {
                    ip4: IP {
                        ip: String::from_utf8(config.ip4.ip.0).expect("valid utf-8"),
                        gw: String::from_utf8(config.ip4.gw.0).expect("valid utf-8"),
                    },
                    ip6: None,
                    domain: None,
                };

                pub_conf.ip6 = match config.ip6 {
                    Some(conf6) => Some(IP {
                        ip: String::from_utf8(conf6.ip.0).expect("valid utf-8"),
                        gw: String::from_utf8(conf6.gw.0).expect("valid utf-8"),
                    }),
                    None => None,
                };

                pub_conf.domain = match config.domain {
                    Some(domain) => Some(String::from_utf8(domain.0).expect("valid utf-8")),
                    None => None,
                };

                Some(pub_conf)
            }
            None => None,
        };

        let interfaces = node
            .interfaces
            .into_iter()
            .map(|intf| {
                let ips = intf.ips.into_iter().map(|ip| String::from_utf8(ip.0).expect("valid utf-8")).collect();
                Interface {
                    name: String::from_utf8(intf.name.0).expect("valid utf-8"),
                    mac: String::from_utf8(intf.mac.0).expect("valid utf-8"),
                    ips
                }
            } 
        )
            .collect();

        let certification = match node.certification {
            DevnetNodeCertification::Certified => NodeCertification::Certified,
            DevnetNodeCertification::Diy => NodeCertification::Diy,
        };

        TfgridNode {
            version: node.version,
            id: node.id,
            farm_id: node.farm_id,
            twin_id: node.twin_id,
            resources,
            location,
            power: Power {
                target: PowerTarget::Up,
                state: PowerState::Up,
                last_uptime: 0,
            },
            public_config,
            created: node.created,
            farming_policy_id: node.farming_policy_id,
            interfaces,
            certification,
            secure_boot: node.secure_boot,
            serial_number: Some(String::from_utf8(node.serial_number).expect("valid utf-8")),
            connection_price: node.connection_price,
        }
    }
}


impl From<TestnetNode> for TfgridNode {
    fn from(node: TestnetNode) -> Self {
        let mut resources = ConsumableResources::default();
        resources.total_resources.cru = node.resources.cru;
        resources.total_resources.hru = node.resources.hru;
        resources.total_resources.mru = node.resources.mru;
        resources.total_resources.sru = node.resources.sru;

        let location = Location {
            city: String::from_utf8(node.city).expect("valid city"),
            country: String::from_utf8(node.country).expect("valid city"),
            latitude: String::from_utf8(node.location.latitude).expect("valid city"),
            longitude: String::from_utf8(node.location.longitude).expect("valid city"),
        };

        let public_config = match node.public_config {
            Some(config) => {
                let mut pub_conf = PublicConfig {
                    ip4: IP {
                        ip: String::from_utf8(config.ip4.ip.0).expect("valid utf-8"),
                        gw: String::from_utf8(config.ip4.gw.0).expect("valid utf-8"),
                    },
                    ip6: None,
                    domain: None,
                };

                pub_conf.ip6 = match config.ip6 {
                    Some(conf6) => Some(IP {
                        ip: String::from_utf8(conf6.ip.0).expect("valid utf-8"),
                        gw: String::from_utf8(conf6.gw.0).expect("valid utf-8"),
                    }),
                    None => None,
                };

                pub_conf.domain = match config.domain {
                    Some(domain) => Some(String::from_utf8(domain.0).expect("valid utf-8")),
                    None => None,
                };

                Some(pub_conf)
            }
            None => None,
        };

        let interfaces = node
            .interfaces
            .into_iter()
            .map(|intf| {
                let ips = intf.ips.into_iter().map(|ip| String::from_utf8(ip.0).expect("valid utf-8")).collect();
                Interface {
                    name: String::from_utf8(intf.name.0).expect("valid utf-8"),
                    mac: String::from_utf8(intf.mac.0).expect("valid utf-8"),
                    ips
                }
            } 
        )
            .collect();

        let certification = match node.certification {
            TestnetNodeCertification::Certified => NodeCertification::Certified,
            TestnetNodeCertification::Diy => NodeCertification::Diy,
        };

        TfgridNode {
            version: node.version,
            id: node.id,
            farm_id: node.farm_id,
            twin_id: node.twin_id,
            resources,
            location,
            power: Power {
                target: PowerTarget::Up,
                state: PowerState::Up,
                last_uptime: 0,
            },
            public_config,
            created: node.created,
            farming_policy_id: node.farming_policy_id,
            interfaces,
            certification,
            secure_boot: node.secure_boot,
            serial_number: Some(String::from_utf8(node.serial_number).expect("valid utf-8")),
            connection_price: node.connection_price,
        }
    }
}


impl From<MainnetNode> for TfgridNode {
    fn from(node: MainnetNode) -> Self {
        let mut resources = ConsumableResources::default();
        resources.total_resources.cru = node.resources.cru;
        resources.total_resources.hru = node.resources.hru;
        resources.total_resources.mru = node.resources.mru;
        resources.total_resources.sru = node.resources.sru;

        let location = Location {
            city: String::from_utf8(node.city).expect("valid city"),
            country: String::from_utf8(node.country).expect("valid city"),
            latitude: String::from_utf8(node.location.latitude).expect("valid city"),
            longitude: String::from_utf8(node.location.longitude).expect("valid city"),
        };

        let public_config = match node.public_config {
            Some(config) => {
                let mut pub_conf = PublicConfig {
                    ip4: IP {
                        ip: String::from_utf8(config.ip4.ip.0).expect("valid utf-8"),
                        gw: String::from_utf8(config.ip4.gw.0).expect("valid utf-8"),
                    },
                    ip6: None,
                    domain: None,
                };

                pub_conf.ip6 = match config.ip6 {
                    Some(conf6) => Some(IP {
                        ip: String::from_utf8(conf6.ip.0).expect("valid utf-8"),
                        gw: String::from_utf8(conf6.gw.0).expect("valid utf-8"),
                    }),
                    None => None,
                };

                pub_conf.domain = match config.domain {
                    Some(domain) => Some(String::from_utf8(domain.0).expect("valid utf-8")),
                    None => None,
                };

                Some(pub_conf)
            }
            None => None,
        };

        let interfaces = node
            .interfaces
            .into_iter()
            .map(|intf| {
                let ips = intf.ips.into_iter().map(|ip| String::from_utf8(ip.0).expect("valid utf-8")).collect();
                Interface {
                    name: String::from_utf8(intf.name.0).expect("valid utf-8"),
                    mac: String::from_utf8(intf.mac.0).expect("valid utf-8"),
                    ips
                }
            } 
        )
            .collect();

        let certification = match node.certification {
            MainnetNodeCertification::Certified => NodeCertification::Certified,
            MainnetNodeCertification::Diy => NodeCertification::Diy,
        };

        TfgridNode {
            version: node.version,
            id: node.id,
            farm_id: node.farm_id,
            twin_id: node.twin_id,
            resources,
            location,
            power: Power {
                target: PowerTarget::Up,
                state: PowerState::Up,
                last_uptime: 0,
            },
            public_config,
            created: node.created,
            farming_policy_id: node.farming_policy_id,
            interfaces,
            certification,
            secure_boot: node.secure_boot,
            serial_number: Some(String::from_utf8(node.serial_number).expect("valid utf-8")),
            connection_price: node.connection_price,
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
