use crate::sources; // 🔥 Import the new sources module
use serde::{Deserialize, Serialize};

// --- RAG SYSTEM (Solana Encyclopedia) ---
// This allows Neurust to answer questions like "How do I use PDAs?" accurately.

pub struct KnowledgeBase {
    entries: Vec<KnowledgeEntry>,
}

struct KnowledgeEntry {
    keywords: Vec<&'static str>,
    content: &'static str,
}

impl KnowledgeBase {
    pub fn new() -> Self {
        Self {
            entries: vec![
                KnowledgeEntry {
                    keywords: vec!["pda", "seeds", "address", "derive", "program derived address"],
                    content: r#"
### SOLANA PDA GUIDE (Program Derived Addresses)
- **Concept:** PDAs are accounts controlled by the program, not a private key.
- **Derivation:** Use `Pubkey::find_program_address(&[seeds], &program_id)`.
- **Anchor Implementation:** Use `#[account(seeds = [b"label", user.key().as_ref()], bump)]` in your struct.
  The `bump` must be stored in the account struct for future validation.
- **Security:** Always validate the `bump` when using the account.
"#,
                },
                KnowledgeEntry {
                    keywords: vec!["cpi", "transfer", "token", "cross program invocation"],
                    content: r#"
### CPI SECURITY (Cross-Program Invocation)
- **Concept:** Calling another program (like the Token Program) from your program.
- **Validation:** Strictly validate the `program_id` of the external program.
- **Anchor Helper:** Use `CpiContext::new(program, accounts)`.
- **Signing:** For PDAs to sign, use `new_with_signer` and pass `&[&[seeds, &[bump]]]`.
"#,
                },
                KnowledgeEntry {
                    keywords: vec!["deployment", "deploy", "mainnet", "devnet", "cost"],
                    content: r#"
### DEPLOYMENT CHECKLIST
1. **Config:** Ensure `Anchor.toml` [programs.localnet] matches the cluster.
2. **Build:** Run `anchor build`. For production, use `anchor build --verifiable`.
3. **Deploy:** `solana program deploy ./target/deploy/your_program.so`.
4. **Buffer Error:** If program is too large, use:
   `solana program write-buffer ./target/deploy/program.so`
   `solana program set-buffer-authority <BUFFER_ADDRESS> --new-authority <YOUR_KEY>`
   `solana program deploy --buffer <BUFFER_ADDRESS>`
"#,
                },
                KnowledgeEntry {
                    keywords: vec!["spl", "token-2022", "mint", "associated token"],
                    content: r#"
### SPL TOKEN & TOKEN-2022
- **Token-2022:** Supports extensions like transfer fees, interest, and non-transferability.
- **Associated Token Account (ATA):** Deterministic address for holding tokens.
  Derive: `get_associated_token_address(wallet, mint)`.
- **Anchor:** Add `associated_token::AssociatedToken` to `#[derive(Accounts)]`.
"#,
                },
            ],
        }
    }

    pub fn search(&self, query: &str) -> String {
        let query_lower = query.to_lowercase();
        let mut results = String::new();

        // 1. Internal Knowledge Search (Quick Snippets)
        for entry in &self.entries {
            for keyword in &entry.keywords {
                if query_lower.contains(keyword) {
                    results.push_str(entry.content);
                    results.push_str("\n---\n");
                    break; // Found a match in this entry
                }
            }
        }

        // 2. 🔥 External Source Recommendation (Enhanced with JSON Data)
        // This scans the data/sources.json file via sources.rs
        let sources = sources::get_trusted_sources();
        let mut suggested_links = String::new();
        
        for source in sources {
            // Fuzzy match against topic, note, or query relevance
            if query_lower.contains(&source.topic.replace("-", " ")) 
               || (source.note.to_lowercase().contains(&query_lower) && query_lower.len() > 3) 
               || query_lower.contains("doc") 
               || query_lower.contains("reference") 
               || query_lower.contains("learn") {
                 
                 let display_name = if !source.note.is_empty() && !source.note.starts_with("---") {
                     format!("{} ({})", source.topic, source.note)
                 } else {
                     source.topic.clone()
                 };

                 suggested_links.push_str(&format!("- [{}]({})\n", display_name, source.url));
            }
        }

        if !suggested_links.is_empty() {
             results.push_str(&format!("\nRECOMMENDED READING (Use 'read_url' action to learn more):\n{}", suggested_links));
        }
        
        if results.is_empty() {
            return String::from("No internal docs found. Suggest using 'read_url' to browse external docs.");
        }
        
        format!("--- 📘 RELEVANT KNOWLEDGE & SOURCES ---\n{}", results)
    }
}

// --- AUTO-HEALING KNOWLEDGE (Error Solutions) ---
// This is fed into the System Prompt so the AI knows how to fix things automatically.

pub fn get_error_solutions() -> &'static str {
    r#"
    --- KNOWLEDGE BASE: COMPREHENSIVE ERROR SOLUTIONS ---

    [STACK: ANCHOR / SOLANA (CRITICAL)]
    
    1. Error: "solana-install no longer supports installing by channel" OR "BPF SDK not found"
       - DIAGNOSIS: The local Solana version is outdated, or the 'init stable' command is deprecated.
       - FIX STRATEGY: Use the official install script to fetch the LATEST STABLE version dynamically.
       - ACTION PLAN:
         {"action": "run_cmd", "program": "sh", "args": ["-c", "curl -sSfL https://release.solana.com/stable/install | sh"]}
       - NOTE: This connects to Solana's release server directly.

    2. Error: "failed to select a version" OR "conflict with previously selected packages" OR "solana-zk-sdk"
       - DIAGNOSIS: Dependency Hell. Versions of 'anchor-spl' (newer) and 'solana-program' (older) are fighting.
       - FIX PLAN (Pinning Versions):
         1. {"action": "run_cmd", "program": "rm", "args": ["Cargo.lock"]}
         // Forcefully add solana-program 1.18.17 to resolve conflicts (This forces downgrades of other deps)
         2. {"action": "run_cmd", "program": "cargo", "args": ["add", "solana-program@=1.18.17"]}
         3. {"action": "run_cmd", "program": "cargo", "args": ["update"]}
         4. {"action": "run_cmd", "program": "anchor", "args": ["build"]}

    3. Error: "lock file version 4 requires" OR "resolver = 1" OR "virtual workspace defaulting to resolver"
       - DIAGNOSIS: Conflict between System Cargo (New) and Solana Toolchain (Old), or missing resolver setting.
       - FIX PLAN (Execute sequentially):
         1. {"action": "run_cmd", "program": "rm", "args": ["Cargo.lock"]} (Remove zombie file)
         2. {"action": "run_cmd", "program": "sh", "args": ["-c", "curl -sSfL https://release.solana.com/stable/install | sh"]} (Update Toolchain)
         3. {"action": "run_cmd", "program": "anchor", "args": ["clean"]}
         4. {"action": "create_file", "path": "Cargo.toml", "content": "[workspace]\nmembers = [\n    \"programs/*\"\n]\nresolver = \"2\"\n\n[profile.release]\noverflow-checks = true\nlto = \"fat\"\ncodegen-units = 1\n[profile.release.build-override]\nopt-level = 3\nincremental = false\ncodegen-units = 1"} 
         5. {"action": "run_cmd", "program": "anchor", "args": ["build"]}

    4. Error: "unresolved import `anchor_spl`" OR "use of unresolved module `anchor_spl`"
       - DIAGNOSIS: Missing `anchor-spl` dependency.
       - FIX: {"action": "run_cmd", "program": "cargo", "args": ["add", "anchor-spl"]}

    5. Error: "Program ID not declared" OR "declare_id!" missing
       - FIX: 
         1. {"action": "run_cmd", "program": "anchor", "args": ["keys", "list"]}
         2. Update `lib.rs` with `declare_id!("YOUR_KEY_HERE");`.

    [STACK: RUST / CARGO]
    1. Error: "linker `cc` not found"
       - FIX: {"action": "run_cmd", "program": "sudo", "args": ["apt", "update", "&&", "sudo", "apt", "install", "build-essential"]}
    
    2. Error: "no such subcommand"
       - FIX: Check spelling. If 'cargo expand' fails, run `cargo install cargo-expand`.

    [STACK: FRONTEND (REACT / NEXT.JS / VITE)]
    1. Error: "yarn: command not found" OR "yarn install failed"
       - FIX: Use `npm` instead. Rewrite command to `npm install` or `npm run dev`.
    
    2. Error: "Hydration failed" (Next.js)
       - FIX: Check for invalid HTML nesting (e.g., <div> inside <p>). Fix the component code.
    
    3. Error: "Module not found: Can't resolve 'fs'"
       - FIX: Add `nodePolyfills` to vite.config.ts.

    4. Error: "EADDRINUSE: address already in use"
       - FIX: Suggest running on port 3001 (e.g., `npm run dev -- --port 3001`).

    [HUMAN FALLBACK PROTOCOL]
    - RULE: If a command fails due to "Network Error", "Permission Denied", or "OS Error" (os error 13):
      1. DO NOT retry the same command blindly.
      2. GENERATE A MESSAGE to the user:
         "I encountered a system limitation (permission/network) while trying to execute a command. Please run this manually in your terminal:
          Command: `[THE_FAILED_COMMAND]`
          Once done, please ask me to continue."
    "#
}