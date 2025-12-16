use crate::services::scraper::ScraperService;
use axum::{Extension, Json}; // Extension Import ပါ
use serde::Deserialize;
use serde_json::{json, Value};
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct ScrapeRequest {
    pub url: String,
    pub topic: String,
}

// Pool ကို Extension ထဲကနေ ဆွဲထုတ်ပါ
pub async fn scrape_handler(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<ScrapeRequest>,
) -> Json<Value> {
    // Pool ကို ScraperService ထဲ ထည့်ပေးပါ
    let scraper = ScraperService::new(pool);

    match scraper.scrape_and_save(&payload.url, &payload.topic).await {
        Ok(content) => Json(json!({
            "status": "success",
            "url": payload.url,
            "content_length": content.len()
        })),
        Err(e) => Json(json!({
            "status": "error",
            "message": e
        })),
    }
}
