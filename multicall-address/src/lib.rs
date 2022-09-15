use ethers::core::types::{Address, Chain};
use once_cell::sync::Lazy;
use serde::Deserialize;

use std::collections::HashMap;

const ADDRESS_JSON: &str = include_str!("./address.json");

static ADDRESS_BOOK: Lazy<HashMap<String, Contract>> =
    Lazy::new(|| serde_json::from_str(ADDRESS_JSON).unwrap());

#[derive(Clone, Debug, Deserialize)]
pub struct Contract {
    addresses: HashMap<Chain, Address>,
}

impl Contract {
    pub fn address(&self, chain: Chain) -> Option<Address> {
        self.addresses.get(&chain).cloned()
    }
}

pub fn contract<S: Into<String>>(name: S) -> Option<Contract> {
    ADDRESS_BOOK.get(&name.into()).cloned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multicall() {
        assert!(contract("multicall2").is_some());
    }

    #[test]
    fn test_address() {
        assert!(contract("multicall2")
            .unwrap()
            .address(Chain::Mainnet)
            .is_some());
        assert!(contract("multicall2")
            .unwrap()
            .address(Chain::Polygon)
            .is_none());
    }
}
