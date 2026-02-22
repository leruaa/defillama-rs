use core::slice;

use defillama::{Chain, Coin, CoinsClient};

#[tokio::test]
async fn current_prices() {
    let client = CoinsClient::default();
    let weth = Coin::Address(
        Chain::Ethereum,
        "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2"
            .parse()
            .unwrap(),
    );

    let prices = client
        .historical_prices(1677266409, slice::from_ref(&weth))
        .await
        .unwrap();

    assert_eq!(prices.len(), 1);
    assert_eq!(prices.get(&weth).unwrap().symbol, "WETH");
}
