use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct StockData {
    pub symbol: String,
    pub price: f64,
    pub market_cap: f64,
    pub volume: f64,
    pub pe_ratio: f64,
}

pub async fn fetch_stock(symbol: &str) -> Result<StockData, String> {
    let client = Client::new();
    let url = format!(
        "https://query1.finance.yahoo.com/v8/finance/chart/{}?interval=1d&range=1d",
        symbol
    );

    let response = client
        .get(&url)
        .header("User-Agent", "Mozilla/5.0")
        .header("Accept", "application/json")
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let json: serde_json::Value = response.json().await.map_err(|e| e.to_string())?;

    // Print the raw response so we can see exactly what Yahoo returns
    println!(
        "Yahoo response: {}",
        serde_json::to_string_pretty(&json).unwrap()
    );

    let meta = &json["chart"]["result"][0]["meta"];

    let price = meta["regularMarketPrice"].as_f64().unwrap_or(0.0);
    let volume = meta["regularMarketVolume"].as_f64().unwrap_or(0.0);
    let market_cap = meta["marketCap"].as_f64().unwrap_or(0.0);

    Ok(StockData {
        symbol: symbol.to_string(),
        price,
        volume,
        market_cap,
        pe_ratio: 0.0,
    })
}
