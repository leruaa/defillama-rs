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

    let prices = client.current_prices(&[weth.clone()]).await.unwrap();

    println!("{prices:?}");
    assert_eq!(prices.len(), 1);
    assert_eq!(prices.get(&weth).unwrap().symbol, "WETH");
}
