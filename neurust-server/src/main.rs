mod models;
mod handlers;
mod knowledge;
mod prompts;
mod services;
mod sources; 
mod middleware;

use axum::{
    routing::{get, post},
    Router, middleware as axum_middleware,
};
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::net::SocketAddr;
use std::sync::Arc;

// 🔥 Import All Services
use services::{
    ai::AiService,
    billing::BillingService, 
    scraper::ScraperService, 
};

use tower_http::cors::{CorsLayer, Any}; 

// 🔥 Import Handlers
// Note: We use explicit paths for user/payment to be clear
use handlers::{
    // Auth Handlers (Device Flow - Public)
    initiate_device_flow, poll_device_flow, verify_device_login, 
    // Agent Handlers (Protected)
    handle_plan_request, handle_audit_request, handle_browse_request, 
    // Project Scaffolding (Protected)
    create_project, delete_project,
    // Health Check (Public)
    health_check,
};

// 🔥 AppState with all services
#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub ai_service: Arc<AiService>,
    pub billing_service: Arc<BillingService>,
    pub scraper_service: Arc<ScraperService>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    println!("🚀 Neurust Server Initializing...");

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("Failed to connect to Postgres");

    println!("✅ Database Connected!");

    // Ensure migrations are up to date
    println!("🔄 Running Migrations...");
    if std::path::Path::new("./migrations").exists() {
        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .expect("Failed to run migrations");
    }

    // 🔥 INITIALIZE SERVICES
    let ai_service = Arc::new(AiService::new(pool.clone()));
    let billing_service = Arc::new(BillingService::new(pool.clone()));
    let scraper_service = Arc::new(ScraperService::new(pool.clone()));

    println!("⏳ Starting Scheduler...");
    // Start the background worker for weekly updates
    services::scheduler::UpdateScheduler::start_weekly_updates(pool.clone()).await;

    // 🔥 Construct State
    let state = AppState {
        pool: pool.clone(),
        ai_service,
        billing_service,
        scraper_service,
    };

    // CORS Layer Setup
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Router Setup
    let app = Router::new()
        // ==========================================
        // 🔐 PROTECTED ROUTES (Require Wallet Auth)
        // ==========================================
        
        // 1. User & Dashboard
        .route("/api/user/me", get(handlers::user::get_me))
        .route("/api/projects", get(handlers::user::get_my_projects))
        
        // 2. Payment (Deposit)
        .route("/api/payment/deposit", post(handlers::payment::top_up_credits))

        // 3. Agent (AI Tools)
        .route("/api/agent/plan", post(handle_plan_request))
        .route("/api/agent/browse", post(handle_browse_request))
        .route("/api/agent/audit", post(handle_audit_request))
        
        // 4. Project Management
        .route("/api/project/create", post(create_project))
        .route("/api/project/delete", post(delete_project))

        // 🔥 Apply Gatekeeper Middleware to ALL routes above
        .layer(axum_middleware::from_fn_with_state(state.clone(), middleware::auth::auth_gatekeeper))

        // ==========================================
        // 🌍 PUBLIC ROUTES (No Auth Required)
        // ==========================================
        
        // 1. Health Check
        .route("/", get(health_check))
        
        // 2. Auth Flow (Login Process)
        .route("/api/auth/device/initiate", post(initiate_device_flow)) 
        .route("/api/auth/device/poll", post(poll_device_flow))       
        .route("/api/auth/device/verify", post(verify_device_login))
        
        // Global Layers
        .layer(cors)
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    println!("🚀 Neurust Brain running on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}