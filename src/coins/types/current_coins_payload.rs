use std::collections::HashMap;

use serde::Deserialize;

use super::{Coin, Price};

#[derive(Debug, Deserialize)]
pub struct CurrentCoinsPayload {
    pub coins: HashMap<Coin, Price>,
}
