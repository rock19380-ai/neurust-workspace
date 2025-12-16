use crate::api::client::ApiClient;
use crate::utils::{cmd, deps, fs};
use anyhow::{anyhow, Result};
use colored::*;
use std::path::Path;

/// CLI Command Entry Point
pub async fn execute(raw_input: String, _project_type: String) -> Result<()> {
    println!("{} Analyzing request: '{}'", "🧠".yellow(), raw_input);

    // AI ကို Plan တောင်းမယ်
    let client = ApiClient::new("http://127.0.0.1:3000".to_string());
    let response = client.fetch_plan(&raw_input, None).await?;

    execute_with_plan(raw_input, response).await
}

/// ask.rs မှ တိုက်ရိုက်ခေါ်သုံးမည့် Function
pub async fn execute_with_plan(raw_input: String, response: serde_json::Value) -> Result<()> {
    let suggested_name = response["suggested_name"]
        .as_str()
        .unwrap_or("neurust_project");
    let project_type = response["project_type"].as_str().unwrap_or("rust");
    let init_cmd = response["init_command"].as_str().unwrap_or("");

    println!("{} AI Suggestion:", "🤖".purple());
    println!("   Name: {}", suggested_name.bold());
    println!("   Type: {}", project_type.cyan());

    // --- Step 1: Initialization (Smart Error Handling) ---

    let parts: Vec<&str> = init_cmd.split_whitespace().collect();
    if let Some((program, args)) = parts.split_first() {
        println!("{} Running init command...", "⚙️".blue());

        // Command Run မယ်
        if let Err(e) = cmd::execute(program, args, None) {
            // Anchor ၏ Yarn Error ကို Soft Fail အနေနဲ့ ကိုင်တွယ်မယ်
            let err_msg = e.to_string();
            let folder_exists = Path::new(suggested_name).exists();

            // အကယ်၍ Folder က ဆောက်ပြီးသွားပြီ၊ ဒါပေမဲ့ Command က Error ပြနေတယ် (ဥပမာ Yarn မရှိလို့)
            // ဒါဆိုရင် မရပ်ဘဲ ဆက်သွားခွင့်ပြုမယ်။
            if folder_exists {
                println!("{} Warning: Init command reported error (likely 'yarn' missing), but folder exists.", "⚠️".yellow());
                println!(
                    "{} Ignoring error and proceeding with Neurust plan (npm install)...",
                    "🔄".cyan()
                );
            } else {
                // Folder လည်း မရှိဘူးဆိုရင်တော့ တကယ် Error တက်တာ
                println!("{} Critical Init Error: {}", "❌".red(), e);
                return Ok(());
            }
        } else {
            println!("{} Init successful!", "✅".green());
        }
    } else {
        // Init command မပါရင် Folder အလွတ်ဆောက်မယ်
        if !Path::new(suggested_name).exists() {
            fs::create_dir(suggested_name)?;
        }
    }

    // --- Step 2: Validate Folder Existence ---
    let project_path = suggested_name.to_string();
    if !Path::new(&project_path).exists() {
        println!(
            "{} Error: Project folder '{}' not found.",
            "❌".red(),
            project_path
        );
        return Ok(());
    }

    // Absolute Path ယူခြင်း
    let abs_project_path = std::fs::canonicalize(Path::new(&project_path))
        .unwrap_or(Path::new(&project_path).to_path_buf())
        .display()
        .to_string();

    println!("{} Project located at: {}", "📂".blue(), abs_project_path);

    // --- Step 3: Execute Plan ---
    println!("{} Configuring Project...", "✨".yellow());

    if let Some(actions) = response["plan"].as_array() {
        for action in actions {
            let action_type = action["action"].as_str().unwrap_or("");

            match action_type {
                "create_file" => {
                    if let Some(rel_path) = action["path"].as_str() {
                        let full_path = format!("{}/{}", project_path, rel_path);

                        // Safety Checks: မလိုအပ်တဲ့ ဖိုင်တွေ ထပ်မဆောက်အောင် ကာကွယ်မယ်
                        if project_type == "frontend"
                            && (rel_path.contains("Cargo.toml") || rel_path.contains("main.rs"))
                        {
                            continue;
                        }
                        if project_type == "anchor" && rel_path.contains("src/main.rs") {
                            continue;
                        }

                        let content = action["content"].as_str().unwrap_or("");
                        if let Err(e) = fs::write_file(&full_path, content) {
                            println!("{} Write Error: {}", "⚠️".yellow(), e);
                        } else {
                            println!("{} Created: {}", "📝".green(), rel_path);
                        }
                    }
                }
                "run_cmd" => {
                    if let Some(program) = action["program"].as_str() {
                        let args: Vec<&str> = action["args"]
                            .as_array()
                            .map(|arr| arr.iter().filter_map(|v| v.as_str()).collect())
                            .unwrap_or_default();

                        println!("{} Executing: {} {:?}", "⚡".yellow(), program, args);

                        // Yarn -> NPM Fallback Logic (AI Plan ထဲမှာ yarn ပါလာခဲ့ရင်)
                        let mut final_program = program;
                        if program == "yarn" {
                            println!(
                                "{} 'yarn' detected via AI plan. Switching to 'npm' for safety...",
                                "🔎".blue()
                            );
                            final_program = "npm";
                        }

                        // Project Folder ထဲဝင်ပြီး Run မယ်
                        match cmd::execute(final_program, &args, Some(&abs_project_path)) {
                            Ok(_) => println!("{} Success", "✅".green()),
                            Err(e) => {
                                println!("{} Task Failed (Non-critical): {}", "⚠️".yellow(), e)
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }

    println!(
        "{} Project '{}' setup complete!",
        "🎉".green(),
        suggested_name
    );
    Ok(())
}
