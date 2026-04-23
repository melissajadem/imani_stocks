use reqwest::Client;
use serde_json::json;

pub async fn analyze_stock(symbol: &str, price: f64, volume: f64) -> Result<String, String> {
    let client = Client::new();
    let api_key =
        std::env::var("CLAUDE_API_KEY").map_err(|_| "Missing CLAUDE_API_KEY".to_string())?;

    let prompt = format!(
        "You are a stock analyst. Analyze this stock and give a clear recommendation.
        
        Stock: {}
        Current Price: ${:.2}
        Volume: {}
        
        Give me:
        1. A brief analysis
        2. A clear recommendation (Buy, Hold or Sell)
        3. Why in 2-3 sentences
        
        Keep it short and direct.",
        symbol, price, volume
    );

    let response = client
        .post("https://api.anthropic.com/v1/messages")
        .header("x-api-key", api_key)
        .header("anthropic-version", "2023-06-01")
        .header("content-type", "application/json")
        .json(&json!({
            "model": "claude-opus-4-6",
            "max_tokens": 1024,
            "messages": [
                {"role": "user", "content": prompt}
            ]
        }))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let json: serde_json::Value = response.json().await.map_err(|e| e.to_string())?;

    println!(
        "Claude response: {}",
        serde_json::to_string_pretty(&json).unwrap()
    );

    let analysis = json["content"][0]["text"]
        .as_str()
        .unwrap_or("No analysis returned")
        .to_string();

    Ok(analysis)
}
