use crate::stocks::StockData;
use reqwest::Client;
use serde_json::json;

pub async fn analyze_stock(
    data: &StockData,
    budget: f64,
    user_news: Option<String>,
) -> Result<String, String> {
    let client = Client::new();
    let api_key =
        std::env::var("CLAUDE_API_KEY").map_err(|_| "Missing CLAUDE_API_KEY".to_string())?;

    let shares_affordable = (budget / data.price).floor();
    let cost = shares_affordable * data.price;

    // Calculate where price sits in 52 week range
    let week_52_range = data.week_52_high - data.week_52_low;
    let price_position = if week_52_range > 0.0 {
        ((data.price - data.week_52_low) / week_52_range * 100.0).round()
    } else {
        0.0
    };

    let news_section = data
        .news
        .iter()
        .enumerate()
        .map(|(i, h)| format!("{}. {}", i + 1, h))
        .collect::<Vec<_>>()
        .join("\n");

    let user_news_section = match &user_news {
        Some(article) => format!("\nUser provided article:\n{}", article),
        None => String::new(),
    };

    let prompt = format!(
        "You are an expert swing trade analyst. Swing trading means holding positions for days to weeks, not minutes or months.

Analyze this stock thoroughly and give a precise, actionable swing trade recommendation.

=== PRICE DATA ===
Symbol: {}
Current Price: ${:.2}
Change Today: {:.2}%
Day Range: ${:.2} - ${:.2}
Previous Close: ${:.2}
52 Week Range: ${:.2} - ${:.2}
52 Week Position: {}% (0% = at 52wk low, 100% = at 52wk high)

=== FUNDAMENTALS ===
P/E Ratio: {:.2}
EPS: ${:.2}
Market Cap: ${:.0}
Sector: {}
Beta: {:.2}
Last Earnings Surprise: {:.2}%

=== VOLUME ===
10 Day Average Volume: {:.0}

=== ANALYST CONSENSUS ===
Strong Buy: {} | Buy: {} | Hold: {} | Sell: {}

=== INSIDER ACTIVITY (last 20 transactions) ===
Insider Buys: {}
Insider Sells: {}

=== RECENT NEWS ===
{}
{}

=== INVESTOR BUDGET ===
Available Budget: ${:.2}
Shares Affordable: {} shares at ${:.2} = ${:.2} total cost

=== YOUR TASK ===
Give a structured swing trade analysis with EXACTLY this format:

VERDICT: [BUY / SELL / HOLD / WAIT FOR PULLBACK]

ENTRY ZONE: $[price] - $[price]
TARGET PRICE: $[price]
STOP LOSS: $[price]
UPSIDE: [%]
DOWNSIDE RISK: [%]
TIMEFRAME: [e.g. 1-2 weeks]
RISK LEVEL: [Low / Medium / High]

SHARES TO BUY: [number] shares at $[entry price] = $[total] of your ${:.2} budget
REMAINING BUDGET: $[amount left over]

TREND: [1 sentence on where price sits vs 52wk range and momentum]
VOLUME: [1 sentence comparing current volume to 10 day average]
FUNDAMENTALS: [1 sentence on valuation and earnings]
INSIDER SIGNAL: [1 sentence on what insider buying/selling suggests]
NEWS SENTIMENT: [1 sentence on recent news impact]

FULL ANALYSIS:
[4-5 sentences of detailed swing trade reasoning. Be specific about price levels, entry timing, catalysts, and risks. Factor in the budget and how many shares they can afford.]

WHAT TO WATCH:
- [Specific price level or event to monitor]
- [Specific price level or event to monitor]
- [Specific price level or event to monitor]

Be direct and honest. If the setup is poor say so clearly. Never give vague advice.",
        data.symbol,
        data.price,
        data.change_percent,
        data.low, data.high,
        data.prev_close,
        data.week_52_low, data.week_52_high,
        price_position,
        data.pe_ratio,
        data.eps,
        data.market_cap,
        data.sector,
        data.beta,
        data.earnings_surprise,
        data.avg_volume_10d,
        data.analyst_strong_buy,
        data.analyst_buy,
        data.analyst_hold,
        data.analyst_sell,
        data.insider_buying,
        data.insider_selling,
        news_section,
        user_news_section,
        budget,
        shares_affordable, data.price, cost,
        budget
    );

    let response = client
        .post("https://api.anthropic.com/v1/messages")
        .header("x-api-key", api_key)
        .header("anthropic-version", "2023-06-01")
        .header("content-type", "application/json")
        .json(&json!({
            "model": "claude-opus-4-6",
            "max_tokens": 1500,
            "messages": [
                {"role": "user", "content": prompt}
            ]
        }))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let json: serde_json::Value = response.json().await.map_err(|e| e.to_string())?;

    let analysis = json["content"][0]["text"]
        .as_str()
        .unwrap_or("No analysis returned")
        .to_string();

    Ok(analysis)
}
