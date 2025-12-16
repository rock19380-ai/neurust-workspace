use colored::*;
use anyhow::Result;
use crate::api::client::ApiClient;
use crate::utils::{fs, context, memory, executor}; 
use std::io::{self, Write};
use std::path::Path;

pub async fn execute(prompt: String) -> Result<()> {
    println!("{} Neurust Agent listening: '{}'", "🤖".purple(), prompt);

    let client = ApiClient::new("http://127.0.0.1:8000".to_string());

    // 1. Memory Load & Smart Context
    let mut mem = memory::ProjectMemory::load();
    
    if mem.project_context.is_empty() {
        println!("{} Initializing Project Memory (One-time Scan)...", "🧠".blue());
        let scanned_context = context::get_project_context();
        mem.project_context = scanned_context;
        let _ = mem.save();
        println!("{} Project structure memorized.", "💾".green());
    } else {
        println!("{} Using cached project memory.", "🧠".blue());
    }

    mem.update_summary(&prompt);
    let _ = mem.save();
    
    // Prepare initial context
    let initial_context_str = format!(
        "{}\n\n--- PROJECT MEMORY (PAST ACTIONS) ---\n{}", 
        mem.project_context, 
        mem.summary
    );
    let mut current_context: Option<String> = Some(initial_context_str);
    
    println!("{} Environment detected & Files loaded.", "🌍".blue());

    let mut conversation_history = String::new(); 

    // --- The Agent Loop (Max 10 Turns) ---
    for turn in 1..=10 {
        println!("{} Consulting Brain (Turn {})...", "🧠".yellow(), turn);
        
        // Browsing Cache Injection
        let cache_file = "neurust_browsing_cache.txt";
        if Path::new(cache_file).exists() {
             if let Ok(content) = fs::read_file(cache_file) {
                 println!("{} Loading browsing cache...", "📖".blue());
                 let mut ctx = current_context.clone().unwrap_or_default();
                 ctx.push_str(&format!("\n\n--- BROWSED CONTENT ---\n{}\n", content));
                 current_context = Some(ctx);
                 let _ = std::fs::remove_file(cache_file);
             }
        }

        let full_prompt = if conversation_history.is_empty() {
            prompt.clone()
        } else {
            format!("ORIGINAL REQUEST: {}\n\nCONVERSATION HISTORY:\n{}", prompt, conversation_history)
        };

        // Call AI Brain
        let response_result = client.fetch_plan(&full_prompt, current_context.clone()).await;
        let response = match response_result {
            Ok(res) => res,
            Err(e) => {
                println!("{} AI Connection Error: {}", "❌".red(), e); 
                return Ok(()); 
            }
        };

        let plan = response["plan"].as_array().map(|v| v.clone()).unwrap_or_default();
        let init_cmd = response["init_command"].as_str().unwrap_or("");

        // 1. AI Chat & User Reply Loop
        if let Some(msg) = response["message"].as_str() {
            if !msg.trim().is_empty() {
                println!("\n{} Neurust: {}", "🤖".green(), msg);
                
                if plan.is_empty() && init_cmd.is_empty() {
                    print!("{} Reply > ", "👤".blue());
                    io::stdout().flush()?;
                    
                    let mut user_reply = String::new();
                    if io::stdin().read_line(&mut user_reply).is_err() { return Ok(()); }
                    let trimmed_reply = user_reply.trim();

                    if trimmed_reply.eq_ignore_ascii_case("exit") || trimmed_reply.eq_ignore_ascii_case("quit") {
                        println!("{} Ending conversation.", "👋".blue());
                        return Ok(());
                    }
                    if trimmed_reply.is_empty() { continue; }

                    mem.update_summary(trimmed_reply);
                    let _ = mem.save();

                    conversation_history.push_str(&format!("\nAI: {}\nUser: {}\n", msg, trimmed_reply));
                    continue; 
                }
            }
        }

        // 2. Reading Logic (Context Update)
        let mut context_updated = false;
        let mut context_buffer = current_context.clone().unwrap_or_default();

        for action in &plan {
            if action["action"] == "read_file" {
                if let Some(path) = action["path"].as_str() {
                    println!("{} Verifying file: {}", "🔎".blue(), path);
                    // Use recursive search if direct read fails
                    let content = fs::read_file(path).ok().or_else(|| {
                         fs::find_file_recursive(path)
                             .and_then(|p| fs::read_file(&p.to_string_lossy()).ok())
                    });

                    if let Some(text) = content {
                        println!("{} Content retrieved.", "✅".green());
                        context_buffer.push_str(&format!("\n\n--- FRESH FILE READ: {} ---\n{}\n", path, text));
                        context_updated = true;
                    } else {
                        println!("{} File not found: {}", "❌".red(), path);
                    }
                }
            }
        }

        if context_updated {
            println!("{} Context updated. Re-thinking...", "🔄".cyan());
            current_context = Some(context_buffer);
            continue; 
        }

        // 3. Execution Phase (Delegated to Executor)
        if !init_cmd.is_empty() {
            println!("{} Handing over to Project Creator...", "🏗️".cyan());
            return executor::smart_create_execute(prompt, response).await;
        }

        if !plan.is_empty() {
            println!("{} Executing Plan...", "⚙️".cyan());
            executor::execute_plan(&response["plan"], &client, &mut mem).await?;
            return Ok(()); 
        }

        if init_cmd.is_empty() && plan.is_empty() { return Ok(()); }
    }

    println!("{} Conversation limit reached.", "🛑".red());
    Ok(())
}