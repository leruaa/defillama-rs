use std::collections::HashMap;

use anyhow::{Context, Result};
use itertools::Itertools;
use reqwest::Client as HttpClient;

use crate::CoinsPayload;

use super::types::{Coin, Price};

pub struct Client {
    base_url: String,
    http_client: HttpClient,
}

impl Client {
    pub async fn current_prices(&self, coins: &[Coin]) -> Result<HashMap<Coin, Price>> {
        let path = coins.iter().map(ToString::to_string).join(",");

        let prices = self
            .http_client
            .get(format!("{}/prices/current/{path}", self.base_url))
            .send()
            .await
            .context("Failed to send current prices request to DefiLlama")?
            .json::<CoinsPayload>()
            .await
            .context("Failed to deserialize DefiLlama current prices response")?;

        Ok(prices.coins)
    }

    pub async fn historical_prices(
        &self,
        timestamp: u64,
        coins: &[Coin],
    ) -> Result<HashMap<Coin, Price>> {
        let path = coins.iter().map(ToString::to_string).join(",");

        let prices = self
            .http_client
            .get(format!(
                "{}/prices/historical/{timestamp}/{path}",
                self.base_url
            ))
            .send()
            .await
            .context("Failed to send historical prices request to DefiLlama")?
            .json::<CoinsPayload>()
            .await
            .context("Failed to deserialize DefiLlama historical prices response")?;

        Ok(prices.coins)
    }
}

impl Default for Client {
    fn default() -> Self {
        Self {
            base_url: String::from("https://coins.llama.fi"),
            http_client: HttpClient::new(),
        }
    }
}
