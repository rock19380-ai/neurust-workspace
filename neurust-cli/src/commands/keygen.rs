use anyhow::Result;
use colored::*;
use solana_sdk::signer::{keypair::Keypair, Signer};
use std::fs;
use std::path::Path;

/// Keypair အသစ်ထုတ်ပြီး ဖိုင်သိမ်းပေးခြင်း
/// path: ဖိုင်သိမ်းမည့်နေရာ (ဥပမာ - "./deploy-key.json")
pub fn generate_and_save_keypair(path_str: &str) -> Result<String> {
    let keypair = Keypair::new();
    let pubkey = keypair.pubkey().to_string();
    let bytes = keypair.to_bytes();

    // JSON array အနေနဲ့ သိမ်းမှ Solana CLI က ဖတ်လို့ရမှာပါ
    let content = serde_json::to_string(&bytes.to_vec())?;

    // Folder မရှိရင် ဆောက်မယ်
    if let Some(parent) = Path::new(path_str).parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(path_str, content)?;

    println!("{} Generated new keypair at: {}", "🔑".yellow(), path_str);
    println!("{} Public Key: {}", "🆔".cyan(), pubkey);

    Ok(pubkey)
}

// CLI Command အနေနဲ့ ခေါ်သုံးရန်
pub async fn execute() -> Result<()> {
    // Default အနေနဲ့ current folder မှာ id.json ထုတ်ပေးမယ်
    generate_and_save_keypair("./id.json")?;
    Ok(())
}
