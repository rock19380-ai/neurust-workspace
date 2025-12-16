use colored::*;
use anyhow::Result;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use std::pin::Pin;
use std::future::Future;
use crate::api::client::ApiClient;
use crate::utils::{fs, cmd, diff, memory};
use crate::utils::diff::ConfirmAction;
use crate::commands::create;

/// Executes the JSON plan returned by the AI.
/// 🔥 FIX: Recursive async calls require explicit Boxing (Pin<Box<...>>) to break infinite size cycles.
pub fn execute_plan<'a>(
    plan_json: &'a serde_json::Value,
    client: &'a ApiClient,
    mem: &'a mut memory::ProjectMemory
) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>> {
    Box::pin(async move {
        // Flag to skip confirmations if user selects "All"
        let mut always_allow = false;

        if let Some(actions) = plan_json.as_array() {
            for action in actions {
                let action_type = action["action"].as_str().unwrap_or("");

                match action_type {
                    "create_file" => {
                        // handle_create_file is synchronous, so no await needed
                        handle_create_file(action, &mut always_allow, mem)?;
                    },
                    "run_cmd" => {
                        handle_run_cmd(action, client, mem).await?;
                    },
                    "read_url" => {
                        handle_read_url(action, client).await?;
                    },
                    _ => {}
                }
            }
        }
        Ok(())
    })
}

/// Handles file creation/update with Diff View & Confirmation
fn handle_create_file(
    action: &serde_json::Value,
    always_allow: &mut bool,
    mem: &mut memory::ProjectMemory
) -> Result<()> {
    if let Some(path) = action["path"].as_str() {
        let new_content = action["content"].as_str().unwrap_or("");
        let reason = action["reason"].as_str().unwrap_or("No reason provided.");
        
        let mut final_path = path.to_string();
        
        // Smart Path Resolution
        if !Path::new(path).exists() {
             if let Some(found) = fs::find_file_recursive(path) {
                 final_path = found.to_string_lossy().to_string();
                 println!("{} Redirecting write to: {}", "🔀".cyan(), final_path);
             }
        }

        let should_write = if *always_allow {
            true
        } else {
            let old_content = if Path::new(&final_path).exists() {
                fs::read_file(&final_path).unwrap_or_default()
            } else {
                String::new()
            };

            match diff::show_diff_and_confirm(&final_path, &old_content, new_content, reason) {
                ConfirmAction::Yes => true,
                ConfirmAction::No => false,
                ConfirmAction::All => {
                    *always_allow = true;
                    true
                }
            }
        };

        if should_write {
            if let Some(parent) = Path::new(&final_path).parent() {
                std::fs::create_dir_all(parent)?;
            }
            println!("{} Updating file: {}", "📝".green(), final_path);
            let _ = fs::write_file(&final_path, new_content);

            // 🔥 MEMORY UPDATE: Smart Context Injection
            mem.append_file_context(&final_path, new_content);
            let _ = mem.save();
        } else {
            println!("{} Skipped updating: {}", "🛑".yellow(), final_path);
        }
    }
    Ok(())
}

/// Handles Command Execution with Auto-Healing Logic
async fn handle_run_cmd(
    action: &serde_json::Value,
    client: &ApiClient,
    mem: &mut memory::ProjectMemory
) -> Result<()> {
    if let Some(program) = action["program"].as_str() {
        let args: Vec<&str> = action["args"].as_array()
            .map(|arr| arr.iter().filter_map(|v| v.as_str()).collect())
            .unwrap_or_default();

        println!("{} Executing: {} {:?}", "⚡".yellow(), program, args);
        
        match cmd::execute_with_output(program, &args, None) {
            Ok(_) => println!("{} Success", "✅".green()),
            Err(e) => {
                println!("{} Command Failed: {}", "❌".red(), e);
                // Call recursive auto-healing
                attempt_auto_healing(program, &args, &e.to_string(), client, mem).await?;
            }
        }
    }
    Ok(())
}

/// Specialized Auto-Healing Subroutine
async fn attempt_auto_healing(
    program: &str,
    args: &[&str],
    error_msg: &str,
    client: &ApiClient,
    mem: &mut memory::ProjectMemory
) -> Result<()> {
    println!("{} Attempting Auto-Healing...", "🩹".yellow());
    
    let fix_prompt = format!(
        "The command '{} {:?}' failed with this error:\n\n{}\n\nAnalyze the error. Provide a JSON plan to fix it immediately.", 
        program, args, error_msg
    );

    if let Ok(fix_response) = client.fetch_plan(&fix_prompt, None).await {
        if let Some(fix_actions) = fix_response["plan"].as_array() {
            if !fix_actions.is_empty() {
                println!("{} Applying Fix...", "🧠".cyan());
                // Recursively call execute_plan for the fix
                // Since execute_plan returns Pin<Box<Future>>, we can await it here.
                return execute_plan(&fix_response["plan"], client, mem).await;
            }
        }
    }
    
    println!("{} Could not fetch auto-healing plan.", "⚠️".yellow());
    Ok(())
}

/// Handles browsing URLs
async fn handle_read_url(action: &serde_json::Value, client: &ApiClient) -> Result<()> {
    if let Some(url) = action["url"].as_str() {
        println!("{} Browsing documentation: {}", "🌐".cyan(), url);
        match client.scrape_url(url).await {
            Ok(content) => {
                println!("{} Page read successfully ({} chars).", "✅".green(), content.len());
                let _ = fs::write_file("neurust_browsing_cache.txt", &content);
                println!("{} Saved to cache.", "💾".blue());
            },
            Err(e) => println!("{} Failed to browse: {}", "❌".red(), e),
        }
    }
    Ok(())
}

/// Handles Smart Project Creation Handover
pub async fn smart_create_execute(prompt: String, response: serde_json::Value) -> Result<()> {
    create::execute_with_plan(prompt, response.clone()).await?;

    let project_name = response["suggested_name"].as_str().unwrap_or("unknown_project");
    let project_path = Path::new(project_name);

    if !project_path.exists() {
        println!("⏳ Waiting for file system...");
        std::thread::sleep(std::time::Duration::from_secs(3));
    }

    if project_path.exists() {
        println!("✅ Project ready at: {}", project_name);
        println!("📂 Please `cd {}` to start working.", project_name);
    } else {
        println!("⚠️ Folder '{}' not found directly.", project_name);
        println!("🔎 Attempting Smart Search...");
        
        if let Some(newest_dir) = find_newest_directory() {
            println!("✅ Found generated project at: {}", newest_dir.display());
            println!("📂 Please `cd {}` to start working.", newest_dir.display());
        } else {
            println!("❌ Could not locate project. Try running manually: {}", response["init_command"].as_str().unwrap_or(""));
        }
    }
    Ok(())
}

fn find_newest_directory() -> Option<PathBuf> {
    let mut entries: Vec<_> = std::fs::read_dir(".")
        .ok()?
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_dir())
        .filter(|entry| !entry.file_name().to_string_lossy().starts_with('.'))
        .collect();

    entries.sort_by_key(|e| e.metadata().ok().and_then(|m| m.modified().ok()).unwrap_or(SystemTime::UNIX_EPOCH));
    entries.last().map(|e| e.path())
}