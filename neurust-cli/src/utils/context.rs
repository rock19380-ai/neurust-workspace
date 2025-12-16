use std::fs;
use std::path::Path;
use std::env;

pub fn get_project_context() -> String {
    let mut context = String::from("\n--- CURRENT WORKING ENVIRONMENT ---\n");
    let current_dir = env::current_dir().unwrap_or_default();
    context.push_str(&format!("Directory: {}\n", current_dir.display()));

    // 1. Rust Project Check
    if Path::new("Cargo.toml").exists() {
        if let Ok(content) = fs::read_to_string("Cargo.toml") {
            let is_workspace = content.contains("[workspace]") && !content.contains("[package]");
            if is_workspace {
                context.push_str("✅ Type: Rust Virtual Workspace (Root)\n");
            } else {
                context.push_str("✅ Type: Rust Single Package\n");
            }
        }
    }

    // 2. Node/Next.js Project Check
    if Path::new("package.json").exists() {
        context.push_str("✅ Type: Node.js / Web Project\n");
    }

    // 3. Anchor Project Check
    if Path::new("Anchor.toml").exists() {
        context.push_str("✅ Type: Solana Anchor Project\n");
    }

    // 🔥🔥🔥 RECURSIVE FILE SCANNING (OPTIMIZED) 🔥🔥🔥
    context.push_str("\n--- 📂 FULL PROJECT SOURCE CODE ---\n");
    let source_code = scan_directory(&current_dir, 0);
    context.push_str(&source_code);

    context
}

/// Recursively reads files, skipping ignored folders and large files.
fn scan_directory(dir: &Path, depth: usize) -> String {
    // Prevent going too deep (Token Saver)
    if depth > 5 { return String::new(); } 

    let mut output = String::new();

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().to_string();

            // 🛑 AGGRESSIVE IGNORE RULES (Token Saving Mode)
            if name.starts_with('.') || // .git, .env, .next, .neurust
               name == "target" || 
               name == "node_modules" || 
               name == "dist" || 
               name == "build" || 
               name == "out" ||
               name == "test-ledger" || // Solana test ledger
               // 🔥 HUGE FILES TO IGNORE
               name.ends_with("package-lock.json") ||
               name.ends_with("yarn.lock") ||
               name.ends_with("Cargo.lock") || 
               name.ends_with(".png") || 
               name.ends_with(".jpg") || 
               name.ends_with(".jpeg") || 
               name.ends_with(".gif") || 
               name.ends_with(".svg") ||
               name.ends_with(".ico") ||
               name.ends_with(".pdf") ||
               name.ends_with(".so") || // Compiled binaries
               name.ends_with(".json") { // Usually config/data, read only if essential
                   // Exception: Read essential config files
                   if name != "package.json" && name != "tsconfig.json" && name != "Anchor.toml" {
                       continue;
                   }
            }

            if path.is_dir() {
                // Recursively scan subdirectories
                output.push_str(&scan_directory(&path, depth + 1));
            } else {
                // Read File Content
                if let Ok(content) = fs::read_to_string(&path) {
                    // 🔥 STRICT SIZE LIMIT: Skip files > 15KB
                    if content.len() < 15_000 {
                        // Get relative path for cleaner context
                        let relative_path = path.strip_prefix(env::current_dir().unwrap_or_default())
                            .unwrap_or(&path)
                            .display();
                        
                        output.push_str(&format!("\n>>>> FILE START: {} <<<<\n{}\n>>>> FILE END: {} <<<<\n", relative_path, content, relative_path));
                    } else {
                        output.push_str(&format!("\n--- FILE: {} (Skipped: Content too large) ---\n", path.display()));
                    }
                }
            }
        }
    }
    output
}