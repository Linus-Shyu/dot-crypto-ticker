# Dot Crypto Ticker

A lightweight Rust application that displays real-time cryptocurrency prices on your Dot device. Perfect for monitoring BTC, ETH, and USDT prices with a clean, compact interface optimized for small screens.

## ✨ Features

- **🪙 Multi-Crypto Support**: Bitcoin (BTC), Ethereum (ETH), and Tether (USDT)
- **📱 Optimized Display**: Mini mode designed specifically for Dot device screens
- **📊 Price Changes**: Real-time 24-hour price change indicators with trend arrows
- **⚡ Fast Updates**: Configurable refresh intervals (default: 10 minutes)
- **🔒 Secure**: Environment-based configuration for API keys
- **🌐 Reliable**: Uses Binance API for accurate, real-time data

## 🚀 Quick Start

### Prerequisites

- Rust 1.70+ installed
- Dot device with Text API content enabled
- Dot API key and device ID

### Installation

1. **Clone the repository:**
```bash
git clone https://github.com/yourusername/dot-crypto-ticker.git
cd dot-crypto-ticker
```

2. **Install dependencies:**
```bash
cargo build
```

3. **Configure your environment:**
```bash
cp env.example .env
```

4. **Edit `.env` with your Dot device credentials:**
```env
DOT_API_KEY=your_dot_api_key_here
DOT_DEVICE_ID=your_device_serial_number_here
DOT_TITLE=Crypto Prices
```

5. **Run the application:**
```bash
cargo run
```

## 📱 Display Format

The application displays cryptocurrency prices in a compact, easy-to-read format:

```
BTC $111,266.93 ↗+2.44%
ETH $4,039.08 ↗+1.23%
USDT $0.9997 ↗+0.01%
```

- **Price formatting**: Includes thousand separators for large numbers
- **Change indicators**: ↗ for gains, ↘ for losses
- **Percentage changes**: Shows 24-hour price change percentage

## ⚙️ Configuration

### Environment Variables

| Variable | Description | Required | Default |
|----------|-------------|----------|---------|
| `DOT_API_KEY` | Your Dot API key from the Dot app | ✅ Yes | - |
| `DOT_DEVICE_ID` | Your Dot device serial number | ✅ Yes | - |
| `DOT_TITLE` | Display title on Dot device | ❌ No | "Crypto Prices" |

### Command Line Options

| Option | Description | Default | Example |
|--------|-------------|---------|---------|
| `--interval-secs` | Update interval in seconds | `600` (10 min) | `--interval-secs 300` |

### Examples

```bash
# Run with default settings (10-minute updates)
cargo run

# Run with 5-minute updates
cargo run -- --interval-secs 300

# Run with 1-minute updates (for testing)
cargo run -- --interval-secs 60
```

## 🔧 Setup Instructions

### 1. Get Your Dot API Credentials

1. Open the **Dot App** on your phone
2. Go to **Content Studio**
3. Add **"Text API content"** to your device
4. Note your **API Key** and **Device Serial Number**

### 2. Configure the Application

1. Copy `env.example` to `.env`
2. Fill in your actual API credentials
3. Optionally customize the display title

### 3. Run the Application

```bash
cargo run
```

The application will start fetching prices and displaying them on your Dot device every 10 minutes.

## 🛠️ Development

### Building

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release
```

### Running Tests

```bash
cargo test
```

### Debug Logging

```bash
RUST_LOG=debug cargo run
```

## 🔄 24/7 Background Running

For continuous operation, you can set up the application to run as a background service on macOS using LaunchAgent.

### Setup Background Service

1. **Build the release version:**
```bash
cargo build --release
```

2. **Create LaunchAgent configuration:**
```bash
# Create LaunchAgents directory if it doesn't exist
mkdir -p ~/Library/LaunchAgents

# Copy the provided plist file to LaunchAgents
cp com.linus.dot-crypto-ticker.plist ~/Library/LaunchAgents/
```

3. **Load and start the service:**
```bash
# Load the service
launchctl load ~/Library/LaunchAgents/com.linus.dot-crypto-ticker.plist

# Start the service
launchctl start com.linus.dot-crypto-ticker
```

### Service Management

| Command | Description |
|---------|-------------|
| `launchctl list \| grep dot-crypto` | Check service status |
| `launchctl start com.linus.dot-crypto-ticker` | Start the service |
| `launchctl stop com.linus.dot-crypto-ticker` | Stop the service |
| `launchctl unload ~/Library/LaunchAgents/com.linus.dot-crypto-ticker.plist` | Unload service completely |

### Logs

- **Normal logs**: `crypto-ticker.log`
- **Error logs**: `crypto-ticker-error.log`

```bash
# View real-time logs
tail -f crypto-ticker.log

# View error logs
tail -f crypto-ticker-error.log
```

### Features

- ✅ **Auto-start**: Starts automatically on system boot
- ✅ **Auto-restart**: Automatically restarts if the application crashes
- ✅ **Background**: Runs in the background without terminal window
- ✅ **Logging**: Comprehensive logging for monitoring and debugging
- ✅ **System Integration**: Uses macOS native service management

## 📋 Requirements

- **Rust**: 1.70 or later
- **Internet**: Required for API calls
- **Dot Device**: With Text API content enabled
- **Memory**: ~8MB RAM usage
- **Storage**: ~2MB disk space

## 🐛 Troubleshooting

### Common Issues

| Issue | Solution |
|-------|----------|
| `Device has no text API content` | Add "Text API content" in Dot App → Content Studio |
| `API key request too frequent` | Increase `--interval-secs` (minimum: 60 seconds) |
| `Failed to fetch from Binance` | Check internet connection and API availability |
| `Missing DOT_API_KEY` | Ensure `.env` file exists with correct API key |

### Getting Help

- 📖 Check the [Issues](https://github.com/yourusername/dot-crypto-ticker/issues) page
- 📚 Review the [Dot API Documentation](https://dot.mindreset.tech/docs/service/studio/api/text_api)
- 💬 Open a new issue for bugs or feature requests

## 🤝 Contributing

Contributions are welcome! Here's how you can help:

1. **Fork** the repository
2. **Create** a feature branch (`git checkout -b feature/amazing-feature`)
3. **Commit** your changes (`git commit -m 'Add amazing feature'`)
4. **Push** to the branch (`git push origin feature/amazing-feature`)
5. **Open** a Pull Request

### Development Guidelines

- Follow Rust naming conventions
- Add tests for new features
- Update documentation as needed
- Keep the code clean and readable

## 📄 License

This project is licensed under the **MIT License** - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [Dot](https://dot.mindreset.tech/) for the amazing device and API
- [Binance](https://binance.com/) for reliable cryptocurrency data
- [Rust](https://rust-lang.org/) community for excellent tooling

## 📈 Roadmap

- [ ] Support for more cryptocurrencies
- [ ] Custom price alerts
- [ ] Historical price charts
- [ ] Multiple Dot device support
- [ ] Web dashboard for configuration

## 📊 Project Stats

![GitHub stars](https://img.shields.io/github/stars/Linus-Shyu/dot-crypto-ticker)
![GitHub forks](https://img.shields.io/github/forks/Linus-Shyu/dot-crypto-ticker)
![GitHub issues](https://img.shields.io/github/issues/Linus-Shyu/dot-crypto-ticker)
![License](https://img.shields.io/github/license/Linus-Shyu/dot-crypto-ticker)

---

**Made with ❤️ for the Dot community**