use anyhow::{Context, Result};
use std::fs;
use toml_edit::{value, DocumentMut}; // Document အစား DocumentMut ကိုသုံးပါ

/// Cargo.toml တွင် Dependency အသစ်ထည့်ခြင်း
pub fn add_dependency(path: &str, crate_name: &str, version: &str) -> Result<()> {
    println!("📦 Adding dependency: {} = \"{}\"", crate_name, version);

    // ၁. ဖိုင်ဖတ်မယ်
    let content = fs::read_to_string(path).context("Could not read Cargo.toml")?;

    // ၂. Parse လုပ်တဲ့အခါ DocumentMut ကို သုံးပါ
    let mut doc = content
        .parse::<DocumentMut>()
        .context("Invalid TOML format")?;

    // ၃. [dependencies] အပိုင်းကို ရှာမယ်
    if doc.get("dependencies").is_none() {
        doc["dependencies"] = toml_edit::table();
    }

    // ၄. Dependency ထည့်မယ်
    doc["dependencies"][crate_name] = value(version);

    // ၅. ပြန်သိမ်းမယ်
    fs::write(path, doc.to_string())?;

    Ok(())
}
