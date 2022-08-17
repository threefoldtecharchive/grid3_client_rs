use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SupportedRuntime {
    Devnet,
    // Testnet,
}

#[derive(Debug, Clone)]
pub struct UnsupportedRuntimeError;

impl FromStr for SupportedRuntime {
    type Err = UnsupportedRuntimeError;

    fn from_str(v: &str) -> Result<Self, Self::Err> {
        match v {
            "devnet" => Ok(Self::Devnet),
            // "testnet" => Ok(Self::Testnet),
            _ => Err(UnsupportedRuntimeError),
        }
    }
}
