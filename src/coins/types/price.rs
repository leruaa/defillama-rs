use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Price {
    pub decimals: u8,
    pub symbol: String,
    pub price: f64,
    pub timestamp: u64,
    pub confidence: f64,
}

#[cfg(feature = "dashu")]
mod dashu {
    use anyhow::Error;
    use dashu_float::{round::mode::HalfAway, DBig, FBig};

    use super::Price;

    impl TryFrom<Price> for DBig {
        type Error = Error;

        fn try_from(value: Price) -> Result<Self, Self::Error> {
            FBig::<HalfAway, 2>::try_from(value.price)
                .map(|d| d.with_base::<10>().value())
                .map_err(Into::into)
        }
    }
}

#[cfg(feature = "dashu")]
pub use dashu::*;
