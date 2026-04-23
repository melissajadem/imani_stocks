use crate::claude::analyze_stock;
use crate::yahoo::fetch_stock;
use axum::{extract::Path, routing::get, Json, Router};

pub fn create_routes() -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/stock/:symbol", get(get_stock))
        .route("/analyze/:symbol", get(analyze))
}

async fn health_check() -> &'static str {
    "Imani Stocks is running!"
}

async fn get_stock(Path(symbol): Path<String>) -> Json<serde_json::Value> {
    match fetch_stock(&symbol).await {
        Ok(data) => Json(serde_json::json!({
            "symbol": data.symbol,
            "price": data.price,
            "volume": data.volume,
            "market_cap": data.market_cap,
        })),
        Err(e) => Json(serde_json::json!({ "error": e })),
    }
}

async fn analyze(Path(symbol): Path<String>) -> Json<serde_json::Value> {
    match fetch_stock(&symbol).await {
        Ok(data) => match analyze_stock(&data.symbol, data.price, data.volume).await {
            Ok(analysis) => Json(serde_json::json!({
                "symbol": data.symbol,
                "price": data.price,
                "analysis": analysis,
            })),
            Err(e) => Json(serde_json::json!({ "error": e })),
        },
        Err(e) => Json(serde_json::json!({ "error": e })),
    }
}
