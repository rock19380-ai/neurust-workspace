use anyhow::{Result, anyhow};
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::{fs, thread, time::Duration};
// use std::env; // 🔥 Remove env import to ensure we don't accidentally use it

use crate::api::client::ApiClient;

pub async fn login() -> Result<()> {
    // 🔥 FORCE PORT 8000: Do not use env vars. Hardcode it to fix the connection.
    let api_url = "http://localhost:8000".to_string();
    
    let client = ApiClient::new(api_url.clone());

    println!("{} (Target: {})", "🔐 Initiating Device Authentication...".cyan(), api_url);

    // 1. Initiate Device Flow
    let flow = match client.initiate_device_flow().await {
        Ok(data) => data,
        Err(e) => {
            // Give a very clear error message
            return Err(anyhow!(
                "Failed to connect to Neurust Server at {}.\n\n👉 PLEASE CHECK:\n1. Is 'cargo run -p neurust-server' running?\n2. Is it listening on Port 8000?\n\n❌ Technical Error: {}", 
                api_url, e
            ));
        }
    };

    // ... (rest of the code remains the same) ...
    // 3. Display Instructions to User
    println!("\n{}", "=".repeat(50).dimmed());
    println!("  1. Open this URL:  {}", flow.verification_uri.blue().underline().bold());
    println!("  2. Enter Code:     {}", flow.user_code.green().bold().on_black().underline());
    println!("{}", "=".repeat(50).dimmed());
    println!();

    // 4. Start Polling UI
    let pb = ProgressBar::new_spinner();
    pb.set_style(ProgressStyle::default_spinner()
        .template("{spinner:.green} {msg}")
        .unwrap()
        .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏"));
    
    pb.set_message("Waiting for browser authorization...");
    pb.enable_steady_tick(Duration::from_millis(100));

    // 5. Polling Loop
    loop {
        thread::sleep(Duration::from_secs(flow.interval));

        match client.poll_device_flow(&flow.device_code).await {
            Ok(res) => {
                match res.status.as_str() {
                    "verified" => {
                        pb.finish_and_clear();
                        if let Some(token) = res.token {
                            fs::write(".neurust_token", &token).ok();
                            println!("{}", "✅ Login Successful!".green().bold());
                            return Ok(());
                        }
                    },
                    "expired" => {
                        pb.finish_with_message("❌ Code expired");
                        return Err(anyhow!("Session expired."));
                    },
                    _ => { pb.set_message("Waiting for browser authorization..."); }
                }
            },
            Err(_) => {
                pb.set_message("Connecting to server...");
            }
        }
    }
}