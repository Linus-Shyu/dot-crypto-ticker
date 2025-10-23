pub async fn fetch_crypto_prices_binance(
    client: &reqwest::Client,
) -> anyhow::Result<Vec<PriceData>> {
    let symbols = vec!["BTCUSDT", "ETHUSDT", "USDCUSDT"]; // BTC, ETH, USDT

    let mut results = Vec::new();

    for symbol in symbols {
        let url = format!(
            "https://api.binance.com/api/v3/ticker/24hr?symbol={}",
            symbol
        );

        #[derive(Deserialize)]
        struct BinanceTicker {
            symbol: String,
            #[serde(rename = "lastPrice")]
            price: String,
            #[serde(rename = "priceChange")]
            price_change: String,
            #[serde(rename = "priceChangePercent")]
            price_change_percent: String,
            volume: String,
            #[serde(rename = "highPrice")]
            high_price: String,
            #[serde(rename = "lowPrice")]
            low_price: String,
        }

        let resp = client.get(&url).send().await.map_err(op);

        match client.get(&url).send().await {
            Ok(resp) => {
                if resp.status().is_success() {
                    match resp.json::<BinanceTicker>().await {
                        Ok(data) => {
                            let crypto_symbol = match symbol {
                                "BTCUSDT" => "BTC",
                                "ETHUSDT" => "ETH",
                                "USDCUSDT" => "USDT",
                                _ => symbol,
                            };

                            let price = data.price.parse().unwrap_or(0.0);
                            info!("Fetched {}: ${}", crypto_symbol, price);

                            results.push(PriceData {
                                symbol: crypto_symbol.to_string(),
                                price,
                                change_24h: data.price_change.parse().ok(),
                                change_percent_24h: data.price_change_percent.parse().ok(),
                                volume_24h: data.volume.parse().ok(),
                                market_cap: None,
                                high_24h: data.high_price.parse().ok(),
                                low_24h: data.low_price.parse().ok(),
                            });
                        }
                        Err(e) => {
                            error!(error = %e, symbol = %symbol, "Failed to parse Binance response");
                        }
                    }
                } else {
                    error!(status = %resp.status(), symbol = %symbol, "Binance API error");
                }
            }
            Err(e) => {
                error!(error = %e, symbol = %symbol, "Failed to fetch from Binance");
            }
        }
    }

    Ok(results)
}
