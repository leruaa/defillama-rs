use std::{fmt::Display, str::FromStr};

use alloy_primitives::Address;
use anyhow::{anyhow, bail, Error, Ok};
use serde_with::DeserializeFromStr;

#[derive(Debug, Clone, DeserializeFromStr, PartialEq, Hash)]
pub enum Coin {
    Address(Chain, Address),
    CoingGecko(String),
}

impl Eq for Coin {}

impl Display for Coin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Coin::Address(chain, address) => write!(f, "{chain}:{address:?}"),
            Coin::CoingGecko(id) => write!(f, "coingecko:{id}"),
        }
    }
}

impl FromStr for Coin {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(':');
        let chain = parts.next().ok_or_else(|| anyhow!("Missing chain"))?;
        let id = parts.next().ok_or_else(|| anyhow!("Missing identifier"))?;

        let coin = match chain {
            "coingecko" => Coin::CoingGecko(String::from(id)),
            chain => Coin::Address(chain.parse()?, id.parse()?),
        };

        Ok(coin)
    }
}

impl TryFrom<Coin> for Address {
    type Error = Error;

    fn try_from(value: Coin) -> Result<Self, Self::Error> {
        match value {
            Coin::Address(_, address) => Ok(address),
            Coin::CoingGecko(id) => {
                bail!("The address can't be retrieved from CoinGecko id '{id}'")
            }
        }
    }
}

#[derive(Debug, Clone, DeserializeFromStr, PartialEq, Hash)]
pub enum Chain {
    Ethereum,
    Bsc,
    Avax,
}

impl Display for Chain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let chain = match self {
            Chain::Ethereum => "ethereum",
            Chain::Bsc => "bsc",
            Chain::Avax => "avax",
        };

        write!(f, "{chain}")
    }
}

impl FromStr for Chain {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ethereum" => Ok(Chain::Ethereum),
            "bsc" => Ok(Chain::Bsc),
            "avax" => Ok(Chain::Avax),
            other => bail!("Chain '{}' not supported", other),
        }
    }
}

impl TryFrom<u64> for Chain {
    type Error = Error;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Chain::Ethereum),
            56 => Ok(Chain::Bsc),
            43114 => Ok(Chain::Avax),
            other => panic!("Chain '{}' not supported", other),
        }
    }
}
