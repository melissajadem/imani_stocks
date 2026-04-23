# Imani Stocks 📈

An open source AI-powered stock analysis dashboard for swing traders. Built with Rust, powered by Claude AI, and Yahoo Finance data.

---

## What it does

- Fetches live stock data (price, volume, 52 week high/low)
- Sends the data to Claude AI for in-depth swing trade analysis
- Returns a clear **Buy / Hold / Sell** recommendation with entry price, target price and stop loss
- Clean web dashboard to search any stock ticker and read the analysis

---

## Tech Stack

- **Backend** — Rust (Axum web server)
- **AI** — Claude API (Anthropic)
- **Stock Data** — Yahoo Finance
- **Frontend** — HTML, CSS, JavaScript

---

## Requirements

- [Rust](https://rustup.rs) installed on your machine
- A Claude API key from [console.anthropic.com](https://console.anthropic.com)

---

## Getting Started

### 1. Clone the repo

```bash
git clone https://github.com/melissajadem/imani_stocks.git
cd imani_stocks
```

### 2. Add your API key

Create a `.env` file in the root of the project:

```
CLAUDE_API_KEY=your_claude_api_key_here
```

Get your Claude API key at [console.anthropic.com](https://console.anthropic.com)

### 3. Build and run

```bash
cargo run
```

### 4. Open the dashboard

Go to [http://localhost:3000](http://localhost:3000) in your browser.

Type any stock ticker (e.g. `AAPL`, `TSLA`, `NVDA`) and click **Analyze**.

---

## Project Structure

```
imani_stocks/
├── src/
│   ├── main.rs        # Server entry point
│   ├── routes.rs      # API endpoints
│   ├── yahoo.rs       # Yahoo Finance data fetching
│   └── claude.rs      # Claude AI analysis
├── frontend/
│   ├── index.html     # Dashboard UI
│   ├── css/
│   │   └── style.css
│   └── js/
│       └── dashboard.js
├── .env               # Your API keys (never commit this)
├── Cargo.toml
└── README.md
```

---

## API Endpoints

| Endpoint               | Description             |
| ---------------------- | ----------------------- |
| `GET /health`          | Check server is running |
| `GET /stock/:symbol`   | Get live stock data     |
| `GET /analyze/:symbol` | Get Claude AI analysis  |

---

## Contributing

Contributions are welcome! This is a free open source project.

1. Fork the repo
2. Create a branch (`git checkout -b feature/your-feature`)
3. Commit your changes (`git commit -m "add your feature"`)
4. Push to your branch (`git push origin feature/your-feature`)
5. Open a Pull Request

Please keep code simple, clean and well commented.

---

## Roadmap

- [ ] Portfolio tracker (start with a set budget e.g. $300)
- [ ] News upload and paste for Claude to read
- [ ] P/E ratio, earnings, market cap from full Yahoo Finance API
- [ ] Price charts
- [ ] Trade history log
- [ ] Email or notification alerts

---

## Disclaimer

This tool is for **educational and informational purposes only**. It is not financial advice. Always do your own research before making any investment decisions.

---

## Author

Made by [@melissajadem](https://github.com/melissajadem)

MIT License — free to use, modify and distribute.
