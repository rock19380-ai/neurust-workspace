use axum::{extract::{Json, State}, http::StatusCode, response::IntoResponse, Extension};
use serde::{Deserialize};
use serde_json::json;
use std::process::Command;
use std::path::Path;
use std::fs;
use std::env; // 🔥 Added env
use crate::{AppState, models::User};

#[derive(Deserialize)]
pub struct CreateProjectRequest {
    pub name: String,
    pub description: Option<String>,
    pub framework: String,
}

#[derive(Deserialize)]
pub struct DeleteProjectRequest {
    pub id: String, 
}

/// Create a new Project
pub async fn create_project(
    State(_state): State<AppState>,
    Extension(user): Extension<User>,
    Json(payload): Json<CreateProjectRequest>,
) -> impl IntoResponse {
    println!("🚀 Scaffolding Project: {} (Framework: {}) for User: {}", 
        payload.name, payload.framework, user.wallet_address);

    // 🔥🔥 FIXED: No more hardcoded "neurust-workspace"
    // 1. Check .env for "PROJECT_ROOT". If not set, use current directory.
    let base_storage_path = env::var("PROJECT_ROOT").unwrap_or_else(|_| ".".to_string());
    
    // 2. Structure: {BASE_PATH}/{WALLET_ADDRESS}/{PROJECT_NAME}
    // Example: /home/abbaas/my-projects/9PR1...iZPQ/my_defi_app
    let user_root = format!("{}/{}", base_storage_path, user.wallet_address);
    let project_path = format!("{}/{}", user_root, payload.name);

    // Ensure user's root directory exists
    if let Err(e) = fs::create_dir_all(&user_root) {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ 
            "status": "error",
            "message": format!("Failed to create user directory at {}: {}", user_root, e) 
        }))).into_response();
    }

    // 3. Determine Command based on Framework
    let (program, args) = match payload.framework.as_str() {
        "Anchor" => ("anchor", vec!["init", &payload.name]),
        "Native" | "Rust" => ("cargo", vec!["new", &payload.name, "--bin"]),
        "NextJS" => ("npx", vec![
            "create-next-app@latest", &payload.name, 
            "--typescript", "--tailwind", "--eslint", 
            "--no-src-dir", "--import-alias", "@/*", 
            "--use-npm", "--yes"
        ]),
        "React" => ("npm", vec!["create", "vite@latest", &payload.name, "--", "--template", "react-ts"]),
        "Tauri" => ("npm", vec![
            "create", "tauri-app@latest", &payload.name, "--", 
            "--template", "react-ts", "--manager", "npm", "--yes"
        ]),
        _ => return (StatusCode::BAD_REQUEST, Json(json!({ "message": "Unsupported framework" }))).into_response(),
    };

    // 4. Execute Scaffolding Command
    // We run this inside `user_root`
    println!("Running: {} {} in {}", program, args.join(" "), user_root);
    let output = Command::new(program)
        .args(&args)
        .current_dir(&user_root) // Run inside the user's specific folder
        .output();

    match output {
        Ok(out) => {
            if !out.status.success() {
                let stderr = String::from_utf8_lossy(&out.stderr);
                // Proceed if directory was created despite warnings
                if !Path::new(&project_path).exists() {
                     return (StatusCode::BAD_REQUEST, Json(json!({ 
                        "status": "error",
                        "message": format!("Scaffolding failed: {}", stderr) 
                    }))).into_response();
                }
            }
        }
        Err(e) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ 
                "status": "error",
                "message": format!("Failed to spawn command: {}", e) 
            }))).into_response();
        }
    }

    // 🔥🔥 5. SMART DEPENDENCY INSTALLER (Yarn -> Npm Fallback)
    if payload.framework == "Anchor" || payload.framework == "React" || payload.framework == "Tauri" {
        println!("📦 Installing Dependencies for {}...", payload.name);
        
        // Try Yarn first
        let yarn_result = Command::new("yarn")
            .arg("install")
            .current_dir(&project_path)
            .status();

        let install_success = match yarn_result {
            Ok(status) if status.success() => true,
            _ => {
                println!("⚠️ Yarn failed or not found. Falling back to NPM...");
                let npm_result = Command::new("npm")
                    .arg("install")
                    .current_dir(&project_path)
                    .status();
                matches!(npm_result, Ok(s) if s.success())
            }
        };

        if !install_success {
             println!("⚠️ Warning: Dependencies failed to install automatically.");
        }
    }

    // 6. Return Success
    Json(json!({
        "status": "success",
        "message": "Project scaffolded successfully",
        "path": project_path,
        "name": payload.name,
        "framework": payload.framework
    })).into_response()
}

/// Delete a Project
pub async fn delete_project(
    State(_state): State<AppState>,
    Extension(user): Extension<User>,
    Json(payload): Json<DeleteProjectRequest>,
) -> impl IntoResponse {
    // 1. Get Base Path (Same Logic)
    let base_storage_path = env::var("PROJECT_ROOT").unwrap_or_else(|_| ".".to_string());
    let user_root = format!("{}/{}", base_storage_path, user.wallet_address);
    
    // Safety check: Don't allow ".." in paths to prevent escaping the directory
    if payload.id.contains("..") || payload.id.contains("/") || payload.id.contains("\\") {
         return (StatusCode::BAD_REQUEST, Json(json!({ "message": "Invalid project name" }))).into_response();
    }

    let project_path = Path::new(&user_root).join(&payload.id);

    if !project_path.exists() {
        return (StatusCode::NOT_FOUND, Json(json!({ "message": "Project not found" }))).into_response();
    }

    match fs::remove_dir_all(&project_path) {
        Ok(_) => Json(json!({ "status": "success", "message": "Project deleted" })).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "message": format!("Failed to delete: {}", e) }))).into_response(),
    }
}