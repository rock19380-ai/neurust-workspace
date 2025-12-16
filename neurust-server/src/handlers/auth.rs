use axum::{extract::{Json, State}, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;
use rand::Rng; 
use crate::AppState; 

// --- Request Models ---

#[derive(Deserialize)]
pub struct DeviceInitRequest {
    #[allow(dead_code)] 
    client_id: Option<String>,
}

#[derive(Deserialize)]
pub struct DevicePollRequest {
    device_code: String,
}

#[derive(Deserialize)]
pub struct DeviceVerifyRequest {
    user_code: String,
    wallet_address: String,
    #[allow(dead_code)]
    message: String,
    #[allow(dead_code)]
    signature: String,
}

#[derive(Serialize)]
pub struct InitiateDeviceFlowResponse {
    device_code: String,
    user_code: String,
    verification_uri: String, 
    expires_in: i64,          
    interval: u64,            
}

// --- Handlers ---

/// 1. Initiate Device Flow (CLI calls this)
pub async fn initiate_device_flow(
    State(state): State<AppState>, 
    Json(_payload): Json<DeviceInitRequest>,
) -> impl IntoResponse {
    let device_code = Uuid::new_v4().to_string();
    let user_code = generate_user_code(); 
    
    let base_url = std::env::var("CLIENT_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());
    let verification_uri = format!("{}/login", base_url);

    // 🔥 FIX 1: Casting 'pending' to the actual Enum Type in Postgres
    let result = sqlx::query!(
        "INSERT INTO device_flows (device_code, user_code, status, expires_at) 
         VALUES ($1, $2, 'pending'::device_code_status, NOW() + INTERVAL '10 minutes')",
        device_code, user_code
    )
    .execute(&state.pool)
    .await;

    match result {
        Ok(_) => {
            Json(InitiateDeviceFlowResponse {
                device_code,
                user_code,
                verification_uri,
                expires_in: 600,
                interval: 5
            }).into_response()
        },
        Err(e) => {
            println!("❌ Database Error (Initiate): {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Database Error").into_response()
        }
    }
}

/// 2. Poll Status (CLI calls this repeatedly)
pub async fn poll_device_flow(
    State(state): State<AppState>, 
    Json(payload): Json<DevicePollRequest>,
) -> impl IntoResponse {
    // 🔥 FIX 2: Force status to act as text for easy comparison in Rust
    let record = sqlx::query!(
        r#"
        SELECT status::text as "status!", wallet_address 
        FROM device_flows 
        WHERE device_code = $1
        "#,
        payload.device_code
    )
    .fetch_optional(&state.pool)
    .await;

    match record {
        Ok(Some(row)) => {
            // "status!" means we force SQLx to treat it as Non-Nullable String
            let status_str = row.status.as_str();

            match status_str {
                "verified" => {
                    let wallet = row.wallet_address.unwrap_or_default();
                    let fake_token = format!("neurust_jwt_{}", wallet);
                    
                    Json(json!({
                        "status": "verified",
                        "token": fake_token,
                        "wallet": wallet
                    })).into_response()
                },
                "expired" => Json(json!({ "status": "expired" })).into_response(),
                _ => Json(json!({ "status": "pending" })).into_response(),
            }
        },
        Ok(None) => {
            (StatusCode::NOT_FOUND, Json(json!({ "error": "Invalid device code" }))).into_response()
        },
        Err(e) => {
            println!("❌ Poll Error: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Database Error").into_response()
        }
    }
}

/// 3. Verify Login (Frontend calls this)
pub async fn verify_device_login(
    State(state): State<AppState>, 
    Json(payload): Json<DeviceVerifyRequest>,
) -> impl IntoResponse {
    println!("🔍 Verifying User: {} Wallet: {}", payload.user_code, payload.wallet_address);

    // 🔥 FIX 3: Casting Enums for Update
    let update_result = sqlx::query!(
        "UPDATE device_flows 
         SET status = 'verified'::device_code_status, wallet_address = $1 
         WHERE user_code = $2 AND status = 'pending'::device_code_status",
        payload.wallet_address, payload.user_code
    )
    .execute(&state.pool)
    .await;

    match update_result {
        Ok(res) => {
            if res.rows_affected() > 0 {
                // 🔥 FIX 4: Added 'role' to Insert (User table likely needs it)
                let _user_reg = sqlx::query!(
                    "INSERT INTO users (wallet_address, credits, role) 
                     VALUES ($1, 100, 'free'::user_role) 
                     ON CONFLICT (wallet_address) DO NOTHING",
                    payload.wallet_address
                )
                .execute(&state.pool)
                .await;

                if let Err(e) = _user_reg {
                    println!("⚠️ User Registration Warning: {}", e);
                } else {
                    println!("✅ User synced with Credit System.");
                }

                println!("✅ Device Verified Successfully!");
                Json(json!({ "status": "success", "message": "Device verified" })).into_response()
            } else {
                println!("⚠️ Verification Failed: Code not found or expired.");
                (StatusCode::BAD_REQUEST, Json(json!({ "message": "Invalid or expired code" }))).into_response()
            }
        },
        Err(e) => {
            println!("❌ Database Error (Verify): {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Database Update Failed").into_response()
        }
    }
}

/// Helper: Generate a numeric user code (e.g., "1234-5678")
fn generate_user_code() -> String {
    let mut rng = rand::rng(); 
    
    let p1: u16 = rng.random_range(1000..10000); 
    let p2: u16 = rng.random_range(1000..10000);
    
    format!("{}-{}", p1, p2)
}