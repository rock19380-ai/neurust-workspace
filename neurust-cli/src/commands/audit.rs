use crate::api::client::ApiClient;
use crate::utils::{fs, cmd};
use colored::*;
use anyhow::Result;
use std::path::Path;

pub async fn execute(path: String) -> Result<()> {
    println!("{}", "🛡️  Starting Deep Security Audit...".cyan().bold());
    println!("{}", "------------------------------------------------".dimmed());

    // 1. Check & Run Dependency Scan (RustSec)
    println!("{}", "🔍 Phase 1: Scanning dependencies (cargo-audit)...".blue());
    
    // Check version silently first (capture output not needed, just success)
    let has_audit_tool = cmd::execute_with_output("cargo", &["audit", "--version"], None).is_ok();

    let audit_json = if has_audit_tool {
        // 🔥 FIX: Use `execute_and_capture` here to get the JSON String
        match cmd::execute_and_capture("cargo", &["audit", "--json"], None) {
            Ok(output) => {
                // If output is essentially empty, assume failure
                if output.trim().is_empty() {
                     r#"{ "status": "scan_failed", "note": "Audit ran but returned empty output" }"#.to_string()
                } else {
                    output
                }
            },
            Err(_) => {
                println!("{}", "⚠️  Dependency scan found issues or failed.".yellow());
                r#"{ "status": "scan_failed_or_issues_found", "note": "Check manual cargo audit output" }"#.to_string()
            }
        }
    } else {
        println!("{}", "⚠️  'cargo-audit' tool not found. Skipping dependency check.".yellow());
        println!("{}", "💡 Tip: Run `cargo install cargo-audit` for deeper scans.".dimmed());
        r#"{ "status": "skipped", "reason": "tool_not_installed" }"#.to_string()
    };

    // 2. Read Source Code
    println!("{} {}", "📖 Phase 2: Reading source code from:".cyan(), path);
    let source_code = match fs::read_file(&path) {
        Ok(code) => code,
        Err(_) => {
            println!("{}", "❌ Error: Could not read file. Audit will be limited.".red());
            String::from("// [ERROR: Source file not found]")
        }
    };

    // 3. Send to Neurust Brain (Hybrid Analysis)
    println!("{}", "🧠 Phase 3: Consulting Neurust Auditor Brain...".magenta());
    
    let client = ApiClient::new("http://127.0.0.1:3000".to_string());
    
    // Data နှစ်ခုကို ပေါင်းပြီး ပို့မယ်
    let combined_input = format!(
        "--- CARGO AUDIT JSON REPORT ---\n{}\n\n--- SOURCE CODE ({}) ---\n{}", 
        audit_json, path, source_code
    );

    // Server ရှိ audit_code endpoint ကို လှမ်းခေါ်မယ်
    match client.audit_code(&combined_input).await {
        Ok(report) => {
            println!("\n{}\n", "=".repeat(60).green());
            println!("{}", report); // Markdown Report Output
            println!("{}\n", "=".repeat(60).green());
            println!("✅ Audit Complete.");
        },
        Err(e) => {
            println!("{} Brain Failure: {}", "❌".red(), e);
        }
    }

    Ok(())
}