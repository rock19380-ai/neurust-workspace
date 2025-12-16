use axum::{extract::{Json, State}, http::StatusCode, response::IntoResponse, Extension};
use serde::{Deserialize};
use serde_json::json;
use crate::{AppState, models::User};

#[derive(Deserialize)]
pub struct DepositRequest {
    pub signature: String, 
    pub amount_sol: f64,   
}

pub async fn top_up_credits(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
    Json(payload): Json<DepositRequest>,
) -> impl IntoResponse {
    println!("💰 Payment Received: User {} sent {} SOL (Sig: {})", 
        user.wallet_address, payload.amount_sol, payload.signature);

    let conversion_rate = 1000.0;
    let credits_to_add = (payload.amount_sol * conversion_rate) as i32;

    if credits_to_add <= 0 {
        return (StatusCode::BAD_REQUEST, Json(json!({ "error": "Invalid amount, too small" }))).into_response();
    }

    // 🔥 FIX 1: Casting $1 to INT4 (Integer)
    let update_result = sqlx::query!(
        "UPDATE users SET credits = credits + $1::int4 WHERE id = $2 RETURNING credits",
        credits_to_add,
        user.id
    )
    .fetch_one(&state.pool)
    .await;

    match update_result {
        Ok(rec) => {
            println!("✅ Credits Added: +{} (New Balance: {})", credits_to_add, rec.credits);
            
            // 🔥 FIX 2: Casting $2 to FLOAT8 (Double Precision) for cost_usd
            let _ = sqlx::query!(
                "INSERT INTO usage_logs (user_id, action, model_used, input_tokens, output_tokens, cost_usd)
                 VALUES ($1, 'deposit', 'solana_pay', 0, 0, $2::FLOAT8)",
                user.id,
                payload.amount_sol 
            )
            .execute(&state.pool)
            .await;

            Json(json!({
                "status": "success",
                "added_credits": credits_to_add,
                "new_balance": rec.credits,
                "message": format!("Successfully added {} credits!", credits_to_add)
            })).into_response()
        },
        Err(e) => {
            println!("❌ Database Error adding credits: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": "Failed to update balance" }))).into_response()
        }
    }
}