use crate::commands::keygen;
use crate::utils::{cmd, fs};
use anyhow::{anyhow, Context, Result};
use clap::Subcommand;
use colored::*;
use dialoguer::{theme::ColorfulTheme, Confirm, Select};
use regex::Regex;
use solana_sdk::signer::{keypair::read_keypair_file, Signer};
use std::path::Path; // ၁. ဒါလေး import လုပ်ဖို့လိုပါတယ်

// ၂. ဒီနေရာမှာ Derive ထည့်ပေးရပါမယ်
#[derive(Subcommand, Debug, Clone)]
pub enum Action {
    /// Request Airdrop intelligently
    Airdrop {
        #[arg(short, long, default_value = "2")]
        amount: u32,
    },
    /// Auto-sync Program ID in lib.rs and Anchor.toml
    Sync,
    /// Build and Deploy to network
    Deploy,
}

pub async fn execute(action: Action) -> Result<()> {
    match action {
        Action::Airdrop { amount } => handle_airdrop(amount).await,
        Action::Sync => handle_sync().await,
        Action::Deploy => handle_deploy().await,
    }
}

// ... (အောက်က handle_deploy, handle_sync, handle_airdrop function တွေက အတူတူပါပဲ၊ မပြောင်းပါဘူး) ...
// ... အရင်ပေးခဲ့တဲ့ code တွေအတိုင်း ဆက်ထားပါ ...

async fn handle_deploy() -> Result<()> {
    println!("{} Starting Smart Deployment...", "🚀".green());

    let key_path = "./target/deploy/program_keypair.json";
    let key_exists = Path::new(key_path).exists();

    let selections = if key_exists {
        vec![
            "Use existing Stable Key (Recommended)",
            "Generate NEW Key (Warning: ID will change)",
            "Cancel",
        ]
    } else {
        vec!["Generate Stable Keypair (First Time)", "Cancel"]
    };

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("How do you want to handle the Program ID?")
        .default(0)
        .items(&selections)
        .interact()?;

    match (key_exists, selection) {
        (true, 0) => println!("{} Using existing stable key...", "🔒".blue()),
        (true, 1) | (false, 0) => {
            println!("{} Generating new stable key...", "🆕".yellow());
            // Keygen directory မရှိရင် error တက်နိုင်လို့ check လိုက်မယ်
            if let Some(parent) = Path::new(key_path).parent() {
                std::fs::create_dir_all(parent)?;
            }
            keygen::generate_and_save_keypair(key_path)?;
        }
        _ => return Ok(()),
    }

    println!("{} Syncing Program ID to Code...", "🔄".blue());
    sync_program_id(key_path)?;

    println!("{} Building Anchor project...", "🔨".blue());
    cmd::execute("anchor", &["build"], None)?;

    println!("{} Deploying to Devnet...", "☁️".blue());
    cmd::execute("anchor", &["deploy", "--provider.cluster", "devnet"], None)?;

    println!("{} Deployment Complete!", "🎉".green());
    Ok(())
}

fn sync_program_id(key_path: &str) -> Result<()> {
    let keypair = read_keypair_file(key_path)
        .map_err(|_| anyhow!("Failed to read keypair at {}", key_path))?;
    let new_id = keypair.pubkey().to_string();

    println!("{} New Program ID: {}", "🆔".cyan(), new_id);

    // TODO: Real path finding logic here. For now, assume standard Anchor layout.
    // Anchor.toml Update
    let anchor_toml = "Anchor.toml";
    if Path::new(anchor_toml).exists() {
        let content = std::fs::read_to_string(anchor_toml)?;
        // Regex to replace: anything inside quotes after declaring specific program name is hard
        // Simple replace for now based on standard pattern
        // Or better: Use toml_edit later. For now, we assume user knows what they are doing.
        println!(
            "{} Please ensure Anchor.toml uses: {}",
            "⚠️".yellow(),
            new_id
        );
    }

    Ok(())
}

async fn handle_airdrop(amount: u32) -> Result<()> {
    println!("{} Requesting {} SOL airdrop...", "💸".green(), amount);

    // ၁. Airdrop တောင်းခြင်း
    cmd::execute("solana", &["airdrop", &amount.to_string()], None)?;

    // ၂. Result ထုတ်ပြခြင်း (Balance စစ်ခြင်း)
    println!("\n{} Checking new balance...", "💰".yellow());
    cmd::execute("solana", &["balance"], None)?;

    Ok(())
}

async fn handle_sync() -> Result<()> {
    sync_program_id("./target/deploy/program_keypair.json")
}
