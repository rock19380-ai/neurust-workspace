pub mod auth;   // Real Logic
pub mod agent;  // AI Logic (Placeholder needed)
pub mod health; // Health Check (Placeholder needed)
pub mod project;
pub mod user;    // User Logic
pub mod payment; // Payment Logic

// Re-export Auth Handlers (Matches main.rs imports)
pub use auth::{initiate_device_flow, poll_device_flow, verify_device_login};

// Re-export Agent Handlers
pub use agent::{handle_plan_request, handle_audit_request, handle_browse_request};

// Re-export Health Check
pub use health::health_check;

// Re-export Project Handlers
pub use project::{create_project, delete_project};
