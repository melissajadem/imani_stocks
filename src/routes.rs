use crate::claude::analyze_stock;
use crate::stocks::fetch_stock;
use axum::{
    extract::Path,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AnalyzeRequest {
    pub budget: f64,
    pub user_news: Option<String>,
}

pub fn create_routes() -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/stock/:symbol", get(get_stock))
        .route("/analyze/:symbol", post(analyze))
}

async fn health_check() -> &'static str {
    "Imani Stocks is running!"
}

async fn get_stock(Path(symbol): Path<String>) -> Json<serde_json::Value> {
    match fetch_stock(&symbol).await {
        Ok(data) => Json(serde_json::json!({
            "symbol": data.symbol,
            "price": data.price,
            "change_percent": data.change_percent,
            "high": data.high,
            "low": data.low,
            "prev_close": data.prev_close,
            "week_52_high": data.week_52_high,
            "week_52_low": data.week_52_low,
            "pe_ratio": data.pe_ratio,
            "eps": data.eps,
            "beta": data.beta,
            "avg_volume_10d": data.avg_volume_10d,
            "market_cap": data.market_cap,
            "sector": data.sector,
            "industry": data.industry,
            "analyst_buy": data.analyst_buy,
            "analyst_hold": data.analyst_hold,
            "analyst_sell": data.analyst_sell,
            "analyst_strong_buy": data.analyst_strong_buy,
            "earnings_surprise": data.earnings_surprise,
            "insider_buying": data.insider_buying,
            "insider_selling": data.insider_selling,
            "news": data.news,
        })),
        Err(e) => Json(serde_json::json!({ "error": e })),
    }
}

async fn analyze(
    Path(symbol): Path<String>,
    Json(body): Json<AnalyzeRequest>,
) -> Json<serde_json::Value> {
    match fetch_stock(&symbol).await {
        Ok(data) => match analyze_stock(&data, body.budget, body.user_news).await {
            Ok(analysis) => Json(serde_json::json!({
                "symbol": data.symbol,
                "price": data.price,
                "change_percent": data.change_percent,
                "sector": data.sector,
                "analyst_buy": data.analyst_buy,
                "analyst_hold": data.analyst_hold,
                "analyst_sell": data.analyst_sell,
                "insider_buying": data.insider_buying,
                "insider_selling": data.insider_selling,
                "news": data.news,
                "analysis": analysis,
            })),
            Err(e) => Json(serde_json::json!({ "error": e })),
        },
        Err(e) => Json(serde_json::json!({ "error": e })),
    }
}
