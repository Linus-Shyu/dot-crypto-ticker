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
