use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Price {
    pub decimals: u8,
    pub symbol: String,
    pub price: f64,
    pub timestamp: u64,
    pub confidence: f64,
}
