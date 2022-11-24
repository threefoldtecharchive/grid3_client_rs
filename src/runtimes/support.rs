use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SupportedRuntime {
    Devnet,
    Testnet,
    Mainnet,
}

#[derive(Debug, Clone)]
pub struct UnsupportedRuntimeError;

impl FromStr for SupportedRuntime {
    type Err = UnsupportedRuntimeError;

    fn from_str(v: &str) -> Result<Self, Self::Err> {
        match v {
            "devnet" => Ok(Self::Devnet),
            "mainnet" => Ok(Self::Mainnet),
            "testnet" => Ok(Self::Testnet),
            _ => Err(UnsupportedRuntimeError),
        }
    }
}
