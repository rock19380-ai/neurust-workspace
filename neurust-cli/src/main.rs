mod api;
mod commands;
mod utils;

use clap::{Parser, Subcommand};
use colored::*;
use commands::{ask, audit, auth, create, solana_cmd};
use utils::repl; 

#[derive(Parser)]
#[command(name = "neurust")]
#[command(about = "Neurust: AI-powered Rust & Solana Engineer", long_about = None)]
struct Cli {
    /// Optional Subcommand (None = Interactive Mode)
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new project
    Create {
        name: String,
        #[arg(short, long, default_value = "solana")]
        r#type: String,
    },
    /// Audit code
    Audit { path: String },
    /// Login to network (Device Flow)
    Login,
    /// Solana DevOps Tools (Airdrop, Sync, Deploy)
    Solana {
        #[command(subcommand)]
        action: solana_cmd::Action,
    },
    /// Ask Neurust AI to do anything (Create, Deploy, Airdrop)
    Ask {
        /// Your prompt (e.g., "Give me 5 SOL", "Create a token")
        prompt: Vec<String>, 
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        // 1. Argument ပါလာရင် Direct Command Run မယ်
        Some(cmd) => {
            if let Err(e) = dispatch_command(cmd).await {
                eprintln!("{} {}", "ERROR:".red().bold(), e);
                std::process::exit(1);
            }
        }
        // 2. Argument မပါရင် Interactive REPL ထဲ ဝင်မယ်
        None => {
            println!("{} Starting Neurust Interactive Shell...", "🚀".cyan());
            if let Err(e) = repl::start().await {
                eprintln!("{} {}", "Shell Error:".red().bold(), e);
            }
        }
    }

    Ok(())
}

/// Command များကို ခွဲဝေလုပ်ဆောင်ပေးမည့် Function
async fn dispatch_command(cmd: Commands) -> anyhow::Result<()> {
    match cmd {
        Commands::Create { name, r#type } => {
            create::execute(name, r#type).await?;
        }
        Commands::Audit { path } => {
            audit::execute(path).await?;
        }
        // 🔥 FIX: auth::execute() အစား auth::login() ကို ပြောင်းခေါ်ထားပါတယ်
        Commands::Login => {
            auth::login().await?; 
        }
        Commands::Solana { action } => {
            solana_cmd::execute(action).await?;
        }
        Commands::Ask { prompt } => {
            // Vec<String> ကို Space ခံပြီး ပြန်ဆက်မယ်
            let prompt_text = prompt.join(" ");
            if !prompt_text.trim().is_empty() {
                ask::execute(prompt_text).await?;
            } else {
                println!("{}", "Please provide a prompt.".yellow());
            }
        }
    }
    Ok(())
}