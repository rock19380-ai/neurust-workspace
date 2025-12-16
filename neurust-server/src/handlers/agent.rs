use crate::AppState;
use crate::models::User; // 🔥 Import User Model
use axum::{
    extract::State, 
    Json, 
    http::StatusCode, 
    response::IntoResponse, 
    Extension // 🔥 Middleware Data ယူရန်
};
use serde_json::{json, Value};

// --- PLANNER HANDLER ---
pub async fn handle_plan_request(
    State(state): State<AppState>, 
    Extension(user): Extension<User>, // 🔥 Gatekeeper ဆီက User အစစ်ကို လက်ခံရယူမယ်
    Json(payload): Json<Value>,    
) -> impl IntoResponse {
    let prompt = payload["prompt"].as_str().unwrap_or("");
    let context = payload["context"].as_str().map(|s| s.to_string());
    
    println!("🤖 User Prompt: {} (Wallet: {})", prompt, user.wallet_address);

    // 1. 💰 PRE-CHECK: Credit လုံလောက်မှု ရှိမရှိ စစ်မယ်
    match state.billing_service.has_sufficient_credits(user.id).await {
        Ok(true) => { /* Proceed */ },
        Ok(false) => {
            return (
                StatusCode::PAYMENT_REQUIRED,
                Json(json!({ "status": "error", "error": "Insufficient Credits. Please top up." }))
            ).into_response();
        },
        Err(e) => {
            println!("❌ Billing Check Error: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR, 
                Json(json!({ "error": "Billing System Error" }))
            ).into_response();
        }
    }

    // 2. 🧠 EXECUTE: AI Service ကို ခေါ်မယ်
    // 🔥 Note: Now capturing 'used_model' (3rd return value)
    match state.ai_service.generate_project_plan(prompt, context).await {
        Ok((plan, usage, used_model)) => {
            println!("✅ Plan generated using [{}]! Usage: {} tokens", used_model, usage.total_tokens);

            // 3. 💸 DEDUCT: ပိုက်ဆံဖြတ်မယ်
            // 🔥 Pass the REAL 'used_model' to billing service so it calculates price accurately
            if let Err(e) = state.billing_service.deduct_credits(
                user.id, 
                &used_model, // <--- Dynamic Model Name (Not hardcoded)
                "generate_plan",
                usage
            ).await {
                println!("❌ Failed to deduct credits: {}", e);
                // Note: Plan is generated, returning it but logging billing failure.
            }

            Json(plan).into_response()
        }
        Err(e) => {
            eprintln!("❌ AI Error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "error",
                    "error": format!("Neurust Brain Failure: {}", e)
                }))
            ).into_response()
        }
    }
}

// --- AUDIT HANDLER ---
pub async fn handle_audit_request(
    State(state): State<AppState>,
    Extension(user): Extension<User>, 
    Json(payload): Json<Value>,
) -> impl IntoResponse {
    let code = payload["code"].as_str().unwrap_or("");
    
    println!("🕵️ Security Audit Request from {} (Size: {} chars)", user.wallet_address, code.len());

    // 1. Credit Check
    match state.billing_service.has_sufficient_credits(user.id).await {
        Ok(true) => {},
        Ok(false) => return (StatusCode::PAYMENT_REQUIRED, Json(json!({ "error": "Insufficient Credits" }))).into_response(),
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": "Billing System Error" }))).into_response(),
    }

    // 2. Execute Audit
    // 🔥 Capturing 'used_model' here too
    match state.ai_service.audit_code(code).await {
        Ok((report, usage, used_model)) => { 
            // 3. Deduct Credits with REAL Model
            let _ = state.billing_service.deduct_credits(
                user.id, 
                &used_model, // <--- Dynamic Model Name
                "audit_code", 
                usage
            ).await;

            Json(json!({ "report": report })).into_response()
        }, 
        Err(e) => {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "report": format!("Error generating audit: {}", e) }))
            ).into_response()
        },
    }
}

// --- BROWSER HANDLER ---
pub async fn handle_browse_request(
    State(state): State<AppState>,
    Extension(user): Extension<User>, 
    Json(payload): Json<Value>
) -> impl IntoResponse {
    let url = payload["url"].as_str().unwrap_or("");
    let topic = payload["topic"].as_str().unwrap_or("User Browsing"); 

    println!("🌐 Browsing Request by {}: {}", user.wallet_address, url);

    // Browsing is currently Free, so no billing call here.
    // If you want to charge for browsing later, add deduct_credits here.

    match state.scraper_service.scrape_and_save(url, topic).await {
        Ok(content) => {
            Json(json!({
                "status": "success",
                "content": content,
                "message": "Content scraped and saved to knowledge base."
            })).into_response()
        },
        Err(e) => {
            println!("❌ Scraper Error: {}", e);
            (
                StatusCode::BAD_REQUEST,
                Json(json!({ "status": "error", "error": e }))
            ).into_response()
        }
    }
}