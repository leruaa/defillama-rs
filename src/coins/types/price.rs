use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Price {
    pub decimals: Option<u8>,
    pub symbol: String,
    pub price: f64,
    pub timestamp: u64,
    pub confidence: f64,
}

#[cfg(feature = "bigdecimal")]
mod bigdecimal {
    use anyhow::Error;
    use bigdecimal::BigDecimal;

    use super::Price;

    impl TryFrom<Price> for BigDecimal {
        type Error = Error;

        fn try_from(value: Price) -> Result<Self, Self::Error> {
            BigDecimal::try_from(value.price).map_err(Into::into)
        }
    }
}
