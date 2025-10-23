use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about = "Crypto Price Ticker for Dot Device - BTC, ETH, USDT")]
pub struct Args {
    /// Refresh interval seconds (default: 600 = 10 minutes)
    #[arg(long, default_value_t = 600)]
    interval_secs: u64,
}

impl Args {
    pub fn interval_secs(&self) -> u64 {
        self.interval_secs
    }
}
