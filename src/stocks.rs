use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StockData {
    pub symbol: String,
    pub price: f64,
    pub change_percent: f64,
    pub high: f64,
    pub low: f64,
    pub prev_close: f64,
    pub week_52_high: f64,
    pub week_52_low: f64,
    pub pe_ratio: f64,
    pub eps: f64,
    pub beta: f64,
    pub avg_volume_10d: f64,
    pub market_cap: f64,
    pub sector: String,
    pub industry: String,
    pub analyst_buy: i64,
    pub analyst_hold: i64,
    pub analyst_sell: i64,
    pub analyst_strong_buy: i64,
    pub earnings_surprise: f64,
    pub insider_buying: i64,
    pub insider_selling: i64,
    pub news: Vec<String>,
}

pub async fn fetch_stock(symbol: &str) -> Result<StockData, String> {
    let client = Client::new();
    let key =
        std::env::var("FINNHUB_API_KEY").map_err(|_| "Missing FINNHUB_API_KEY".to_string())?;

    // 1. Quote - price, change, high, low, prev close
    let quote = client
        .get(format!(
            "https://finnhub.io/api/v1/quote?symbol={}&token={}",
            symbol, key
        ))
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json::<serde_json::Value>()
        .await
        .map_err(|e| e.to_string())?;

    // 2. Basic financials - P/E, EPS, beta, 52wk, avg volume
    let metrics = client
        .get(format!(
            "https://finnhub.io/api/v1/stock/metric?symbol={}&metric=all&token={}",
            symbol, key
        ))
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json::<serde_json::Value>()
        .await
        .map_err(|e| e.to_string())?;

    // 3. Company profile - market cap, sector, industry
    let profile = client
        .get(format!(
            "https://finnhub.io/api/v1/stock/profile2?symbol={}&token={}",
            symbol, key
        ))
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json::<serde_json::Value>()
        .await
        .map_err(|e| e.to_string())?;

    // 4. Analyst recommendations
    let recommendations = client
        .get(format!(
            "https://finnhub.io/api/v1/stock/recommendation?symbol={}&token={}",
            symbol, key
        ))
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json::<serde_json::Value>()
        .await
        .map_err(|e| e.to_string())?;

    // 5. Earnings surprise - last 4 quarters
    let earnings = client
        .get(format!(
            "https://finnhub.io/api/v1/stock/earnings?symbol={}&token={}",
            symbol, key
        ))
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json::<serde_json::Value>()
        .await
        .map_err(|e| e.to_string())?;

    // 6. Insider transactions
    let insider = client
        .get(format!(
            "https://finnhub.io/api/v1/stock/insider-transactions?symbol={}&token={}",
            symbol, key
        ))
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json::<serde_json::Value>()
        .await
        .map_err(|e| e.to_string())?;

    // 7. Company news from Finnhub - last 7 days
    let today = chrono::Utc::now();
    let week_ago = today - chrono::Duration::days(7);
    let to_date = today.format("%Y-%m-%d").to_string();
    let from_date = week_ago.format("%Y-%m-%d").to_string();

    let news = client
        .get(format!(
            "https://finnhub.io/api/v1/company-news?symbol={}&from={}&to={}&token={}",
            symbol, from_date, to_date, key
        ))
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json::<serde_json::Value>()
        .await
        .map_err(|e| e.to_string())?;

    // Parse quote
    let price = quote["c"].as_f64().unwrap_or(0.0);
    let change_percent = quote["dp"].as_f64().unwrap_or(0.0);
    let high = quote["h"].as_f64().unwrap_or(0.0);
    let low = quote["l"].as_f64().unwrap_or(0.0);
    let prev_close = quote["pc"].as_f64().unwrap_or(0.0);

    // Parse metrics
    let m = &metrics["metric"];
    let pe_ratio = m["peNormalizedAnnual"].as_f64().unwrap_or(0.0);
    let eps = m["epsNormalizedAnnual"].as_f64().unwrap_or(0.0);
    let week_52_high = m["52WeekHigh"].as_f64().unwrap_or(0.0);
    let week_52_low = m["52WeekLow"].as_f64().unwrap_or(0.0);
    let beta = m["beta"].as_f64().unwrap_or(0.0);
    let avg_volume_10d = m["10DayAverageTradingVolume"].as_f64().unwrap_or(0.0) * 1_000_000.0;

    // Parse profile
    let market_cap = profile["marketCapitalization"].as_f64().unwrap_or(0.0) * 1_000_000.0;
    let sector = profile["finnhubIndustry"]
        .as_str()
        .unwrap_or("Unknown")
        .to_string();
    let industry = profile["name"].as_str().unwrap_or("Unknown").to_string();

    // Parse analyst recommendations - get latest period
    let rec = &recommendations[0];
    let analyst_buy = rec["buy"].as_i64().unwrap_or(0);
    let analyst_hold = rec["hold"].as_i64().unwrap_or(0);
    let analyst_sell = rec["sell"].as_i64().unwrap_or(0);
    let analyst_strong_buy = rec["strongBuy"].as_i64().unwrap_or(0);

    // Parse earnings surprise - last quarter
    let earnings_surprise = earnings[0]["surprisePercent"].as_f64().unwrap_or(0.0);

    // Parse insider transactions - count buys vs sells
    let mut insider_buying = 0i64;
    let mut insider_selling = 0i64;

    if let Some(transactions) = insider["data"].as_array() {
        for t in transactions.iter().take(20) {
            let change = t["change"].as_i64().unwrap_or(0);
            if change > 0 {
                insider_buying += 1;
            } else if change < 0 {
                insider_selling += 1;
            }
        }
    }

    // Parse news headlines - take top 5
    let news_headlines: Vec<String> = news
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .take(5)
        .map(|a| a["headline"].as_str().unwrap_or("").to_string())
        .filter(|h| !h.is_empty())
        .collect();

    Ok(StockData {
        symbol: symbol.to_string(),
        price,
        change_percent,
        high,
        low,
        prev_close,
        week_52_high,
        week_52_low,
        pe_ratio,
        eps,
        beta,
        avg_volume_10d,
        market_cap,
        sector,
        industry,
        analyst_buy,
        analyst_hold,
        analyst_sell,
        analyst_strong_buy,
        earnings_surprise,
        insider_buying,
        insider_selling,
        news: news_headlines,
    })
}
