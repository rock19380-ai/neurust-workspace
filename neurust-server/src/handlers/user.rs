use axum::{Json, response::IntoResponse, Extension};
use serde_json::{json, Value};
use crate::models::User;

pub async fn get_me(Extension(user): Extension<User>) -> impl IntoResponse {
    Json(json!({
        "id": user.id,
        "wallet_address": user.wallet_address,
        "credits": user.credits, // 🔥 This is what Dashboard needs
        "role": user.role
    }))
}

pub async fn get_my_projects(Extension(user): Extension<User>) -> impl IntoResponse {
    // For now, return empty or mock. Connect to DB later.
    // Dashboard expects an array
    Json(json!([])).into_response()
}