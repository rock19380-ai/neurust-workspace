use crate::{AppState, models::{User, UserRole}};
use axum::{
    body::Body,
    extract::{State, Request},
    http::{StatusCode},
    middleware::Next,
    response::Response,
};
use sqlx::Row;
use std::env;

// Middleware Function
pub async fn auth_gatekeeper(
    State(state): State<AppState>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    
    // 1. Header ထဲက "x-neurust-wallet" ကို ရှာမယ်
    let wallet_address = match req.headers().get("x-neurust-wallet") {
        Some(value) => value.to_str().unwrap_or("").to_string(),
        None => {
            println!("⛔ Auth Failed: No Wallet Header found");
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    println!("🔐 Gatekeeper Checking: {}", wallet_address);

    // 🔥 Check .env for Super Admin Wallet
    let super_admin_wallet = env::var("SUPER_ADMIN_WALLET").unwrap_or_default();
    let is_super_admin = wallet_address == super_admin_wallet && !super_admin_wallet.is_empty();

    // 2. Database User Lookup (Find or Create)
    let user = match sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE wallet_address = $1"
    )
    .bind(&wallet_address)
    .fetch_optional(&state.pool)
    .await 
    {
        Ok(Some(mut user)) => {
            // 🔥 Security Update: If wallet matches env var, FORCE promote to Super Admin
            if is_super_admin && user.role != UserRole::SuperAdmin {
                println!("👑 Promoting User to Super Admin via Env Var");
                let _ = sqlx::query("UPDATE users SET role = 'super_admin' WHERE id = $1")
                    .bind(user.id)
                    .execute(&state.pool).await;
                user.role = UserRole::SuperAdmin;
            }
            user
        }, 
        Ok(None) => {
            println!("🆕 New User detected! Registering: {}", wallet_address);
            
            // 🔥 Auto-Assign Role & Credits based on .env
            let (role_str, start_credits) = if is_super_admin { 
                ("super_admin", 999999) 
            } else { 
                ("free", 50) 
            };

            let new_user = sqlx::query_as::<_, User>(
                "INSERT INTO users (wallet_address, role, credits) 
                 VALUES ($1, $2::user_role, $3) 
                 RETURNING *"
            )
            .bind(&wallet_address)
            .bind(role_str)
            .bind(start_credits)
            .fetch_one(&state.pool)
            .await
            .map_err(|e| {
                eprintln!("❌ DB Error creating user: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;
            new_user
        },
        Err(e) => {
            eprintln!("❌ DB Error fetching user: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    // 3. User Info ကို Request Context ထဲ ထည့်ပေးလိုက်မယ်
    req.extensions_mut().insert(user);

    Ok(next.run(req).await)
}