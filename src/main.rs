use anyhow::{Context, Result};
use clap::Parser;
use dotenvy::dotenv;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use std::env;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{error, info};

#[derive(Parser, Debug)]
#[command(version, about = "Crypto Price Ticker for Dot Device - BTC, ETH, USDT")] 
struct Args {
    /// Refresh interval seconds (default: 600 = 10 minutes)
    #[arg(long, default_value_t = 600)]
    interval_secs: u64,
}

#[derive(Serialize)]
struct DotTextPayload<'a> {
    #[serde(rename = "refreshNow", skip_serializing_if = "Option::is_none")]
    refresh_now: Option<bool>,
    #[serde(rename = "deviceId")]
    device_id: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    signature: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    link: Option<&'a str>,
}

#[derive(Debug, Clone)]
struct PriceData {
    symbol: String,
    price: f64,
    change_24h: Option<f64>,
    change_percent_24h: Option<f64>,
    volume_24h: Option<f64>,
    market_cap: Option<f64>,
    high_24h: Option<f64>,
    low_24h: Option<f64>,
}

async fn fetch_crypto_prices_binance(client: &reqwest::Client) -> Result<Vec<PriceData>> {
    let symbols = vec!["BTCUSDT", "ETHUSDT", "USDCUSDT"]; // BTC, ETH, USDT

    let mut results = Vec::new();

    for symbol in symbols {
        let url = format!("https://api.binance.com/api/v3/ticker/24hr?symbol={}", symbol);
        
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


async fn send_to_dot_text_api(
    client: &reqwest::Client, 
    api_key: &str, 
    device_id: &str, 
    title: &str, 
    message: String, 
    signature: String
) -> Result<()> {
    let payload = DotTextPayload {
        refresh_now: Some(true),
        device_id: device_id,
        title: Some(title),
        message: Some(message),
        signature: Some(signature),
        icon: None,
        link: None,
    };
    
    let res = client
        .post("https://dot.mindreset.tech/api/open/text")
        .header(AUTHORIZATION, format!("Bearer {}", api_key))
        .header(CONTENT_TYPE, "application/json")
        .json(&payload)
        .send()
        .await
        .context("send dot api")?;
    
    let status = res.status();
    let body = res.text().await.unwrap_or_default();
    
    if !status.is_success() {
        error!(%status, body, "Dot API error");
        anyhow::bail!("dot api status {}", status);
    }
    
    info!(%status, body, "Dot API ok");
    Ok(())
}

fn format_price(price: f64) -> String {
    if price >= 1000.0 {
        let formatted = format!("{:.2}", price);
        let parts: Vec<&str> = formatted.split('.').collect();
        let integer_part = parts[0];
        let decimal_part = parts[1];
        
        let mut result = String::new();
        let chars: Vec<char> = integer_part.chars().collect();
        for (i, c) in chars.iter().enumerate() {
            if i > 0 && (chars.len() - i) % 3 == 0 {
                result.push(',');
            }
            result.push(*c);
        }
        result.push('.');
        result.push_str(decimal_part);
        format!("${}", result)
    } else {
        format!("${:.2}", price)
    }
}

fn format_change(change_percent: f64) -> String {
    let sign = if change_percent >= 0.0 { "+" } else { "" };
    let arrow = if change_percent >= 0.0 { "↗" } else { "↘" };
    format!("{}{}{:.1}%", arrow, sign, change_percent)
}

fn create_display_message(data: &[PriceData]) -> String {
    // Ultra compact mode for small screens - show BTC, ETH, USDT all in one page
    let mut lines = Vec::new();
    
    // Sort coins by priority: BTC first, then ETH, then USDT, then others
    let mut sorted_data = data.to_vec();
    sorted_data.sort_by(|a, b| {
        let priority = |symbol: &str| match symbol {
            "BTC" => 0,
            "ETH" => 1,
            "USDT" => 2,
            _ => 3,
        };
        priority(&a.symbol).cmp(&priority(&b.symbol))
    });
    
    // Show all 3 main coins (BTC, ETH, USDT) in compact format
    for coin in sorted_data.iter().take(3) {
        let price_str = format_price(coin.price);
        let change_str = if coin.change_percent_24h.is_some() {
            let change_percent = coin.change_percent_24h.unwrap();
            format_change(change_percent)
        } else {
            "".to_string()
        };
        
        if change_str.is_empty() {
            lines.push(format!("{} {}", coin.symbol, price_str));
        } else {
            lines.push(format!("{} {} {}", coin.symbol, price_str, change_str));
        }
    }
    lines.join("\n")
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let args = Args::parse();
    let interval = args.interval_secs.max(2);

    let api_key = env::var("DOT_API_KEY").context("missing DOT_API_KEY")?;
    let device_id = env::var("DOT_DEVICE_ID").context("missing DOT_DEVICE_ID")?;
    let title = env::var("DOT_TITLE").unwrap_or_else(|_| "Crypto Prices".to_string());

    let http = reqwest::Client::builder()
        .user_agent("dot-crypto-ticker/0.2")
        .timeout(Duration::from_secs(30))
        .build()
        .context("build client")?;

    info!("Starting crypto price ticker - BTC, ETH, USDT - 10 minute intervals");

    loop {
        match fetch_crypto_prices_binance(&http).await {
            Ok(data) => {
                if !data.is_empty() {
                    let message = create_display_message(&data);
                    let signature = format!("Updated at {}", chrono::Local::now().format("%H:%M"));
                    
                    if let Err(e) = send_to_dot_text_api(&http, &api_key, &device_id, &title, message, signature).await {
                        error!(error = %e, "failed to send to dot api");
                    } else {
                        let symbols: Vec<String> = data.iter().map(|d| d.symbol.clone()).collect();
                        info!("Prices updated: {:?}", symbols);
                    }
                } else {
                    error!("No price data received");
                }
            }
            Err(e) => {
                error!(error = %e, "price fetch failed");
            }
        }

        sleep(Duration::from_secs(interval)).await;
    }
}