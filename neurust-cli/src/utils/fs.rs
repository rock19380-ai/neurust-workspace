use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// ဖိုင်တစ်ခု ဖန်တီးခြင်း (Write File)
/// လမ်းကြောင်းပေါ်ရှိ Folder များ မရှိသေးလျှင် အလိုအလျောက် ဆောက်ပေးသည်။
pub fn write_file(path: &str, content: &str) -> Result<()> {
    let path_obj = Path::new(path);

    // ၁. Parent Directory ရှိမရှိစစ်၊ မရှိရင် ဆောက်
    if let Some(parent) = path_obj.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create directory: {:?}", parent))?;
        }
    }

    // ၂. ဖိုင်ကို ရေး (Write)
    fs::write(path, content).with_context(|| format!("Failed to write to file: {}", path))?;

    Ok(())
}

/// ဖိုင်တစ်ခုကို ဖတ်ခြင်း (Read File)
pub fn read_file(path: &str) -> Result<String> {
    fs::read_to_string(path).with_context(|| format!("Failed to read file: {}", path))
}

/// Folder အသစ် ဆောက်ခြင်း
pub fn create_dir(path: &str) -> Result<()> {
    fs::create_dir_all(path).with_context(|| format!("Failed to create directory: {}", path))
}

/// ဖိုင်နာမည် (သို့) Path အပိုင်းအစကို ပေးလိုက်ရင် တကယ့် Path ကို ရှာပေးခြင်း
pub fn find_file_recursive(target_path_str: &str) -> Option<PathBuf> {
    let target_path = Path::new(target_path_str);
    let target_file_name = target_path.file_name()?;

    // လက်ရှိ Folder အောက်က ဖိုင်အားလုံးကို လိုက်ရှာမယ်
    for entry in WalkDir::new(".")
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let entry_path = entry.path();

        // Performance: target folder နဲ့ .git တွေကို မရှာဘူး (ကျော်မယ်)
        if entry_path.to_string_lossy().contains("/target/")
            || entry_path.to_string_lossy().contains("/.git/")
            || entry_path.to_string_lossy().contains("/node_modules/")
        {
            continue;
        }

        // ၁. Path အတိအကျတူရင် (Perfect Match)
        if entry_path.ends_with(target_path) {
            return Some(entry_path.to_path_buf());
        }

        // ၂. Filename တူရင် (Fuzzy Match) - ဥပမာ src/lib.rs လို့ရှာရင် programs/app/src/lib.rs ကိုတွေ့အောင်
        if let Some(fname) = entry_path.file_name() {
            if fname == target_file_name {
                // အကယ်၍ target path က path အပိုင်းအစ (partial path) ဖြစ်နေရင်လည်း စစ်မယ်
                if entry_path.to_string_lossy().ends_with(target_path_str) {
                    return Some(entry_path.to_path_buf());
                }
            }
        }
    }
    None
}
