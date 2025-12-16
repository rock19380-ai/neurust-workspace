use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use anyhow::Result;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ProjectMemory {
    pub project_name: String,
    pub last_interaction: String, // Timestamp
    pub summary: String,          // Project အကြောင်း အကျဉ်းချုပ် (Chat History)
    pub pending_tasks: Vec<String>, // လက်ကျန်အလုပ်များ
    pub active_files: Vec<String>,  // နောက်ဆုံးပြင်ခဲ့တဲ့ ဖိုင်များ
    
    // 🔥 NEW: Context Cache for "Scan Once" logic
    // ဒီ Field က Project တစ်ခုလုံးရဲ့ File Content တွေကို String အကြီးကြီးအနေနဲ့ သိမ်းထားပါမယ်။
    // ဒါရှိနေရင် CLI က နောက်တစ်ခါ Disk ကို Scan မဖတ်တော့ပါဘူး။
    #[serde(default)] 
    pub project_context: String, 
}

impl ProjectMemory {
    /// Memory ဖိုင်လမ်းကြောင်း (.neurust/memory.json)
    fn get_path() -> PathBuf {
        Path::new(".neurust").join("memory.json")
    }

    /// Memory ကို ဖတ်မယ် (မရှိရင် အသစ်ဆောက်မယ်)
    pub fn load() -> Self {
        let path = Self::get_path();
        if path.exists() {
            if let Ok(content) = fs::read_to_string(&path) {
                if let Ok(mem) = serde_json::from_str::<ProjectMemory>(&content) {
                    return mem;
                }
            }
        }
        // မရှိရင် Default ပြန်ပေးမယ်
        Self::default()
    }

    /// Memory ကို သိမ်းမယ်
    pub fn save(&self) -> Result<()> {
        let dir = Path::new(".neurust");
        if !dir.exists() {
            fs::create_dir(dir)?;
        }
        
        let content = serde_json::to_string_pretty(self)?;
        fs::write(Self::get_path(), content)?;
        Ok(())
    }

    /// Chat History ကို Update လုပ်မယ်
    pub fn update_summary(&mut self, last_prompt: &str) {
        // Timestamp ထည့်မယ်
        self.last_interaction = chrono::Local::now().to_rfc3339();
        
        if !self.summary.is_empty() {
            self.summary.push_str("\n");
        }
        // Summary ကို အရမ်းရှည်မသွားအောင် ထိန်းမယ် (Logic အကြမ်း)
        let new_entry = format!("- Task: {}\n", last_prompt);
        self.summary.push_str(&new_entry);
    }

    /// 🔥 NEW: Smart Context Update
    /// ဖိုင်အသစ်ဆောက်လိုက်တဲ့အခါ၊ Disk ကို ပြန် Scan ဖတ်စရာမလိုဘဲ
    /// Memory ထဲက Context ကို တိုက်ရိုက် လှမ်းဖြည့်ပေးလိုက်တဲ့ Function ပါ။
    pub fn append_file_context(&mut self, path: &str, content: &str) {
        self.project_context.push_str(&format!("\n>>>> FILE START: {} <<<<\n{}\n>>>> FILE END: {} <<<<\n", path, content, path));
    }
}