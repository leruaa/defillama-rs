use std::collections::HashMap;

use serde::Deserialize;

use super::{Coin, Price};

#[derive(Debug, Deserialize)]
pub struct CoinsPayload {
    pub coins: HashMap<Coin, Price>,
}
