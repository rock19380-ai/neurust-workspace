use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Source {
    #[serde(default)]
    pub note: String,
    pub url: String,
    pub topic: String,
}

pub fn get_trusted_sources() -> Vec<Source> {
    // 🔥 Path (၃) မျိုး စမ်းရှာခိုင်းမယ် (Error ကာကွယ်ဖို့)
    // 1. data/sources.json (Standard)
    // 2. neurust-server/data/sources.json (If running from workspace root)
    // 3. ../data/sources.json (Fallback)
    let potential_paths = [
        "data/sources.json",                
        "neurust-server/data/sources.json", 
        "../data/sources.json",             
    ];

    for path_str in potential_paths {
        let path = Path::new(path_str);
        
        // ဖိုင်ရှိမှ ဆက်လုပ်မယ်
        if path.exists() {
            println!("📂 Found sources file at: {:?}", path); 
            
            if let Ok(content) = fs::read_to_string(path) {
                match serde_json::from_str::<Vec<Source>>(&content) {
                    Ok(sources) => {
                        println!("✅ Successfully loaded {} sources from JSON.", sources.len());
                        return sources;
                    },
                    Err(e) => eprintln!("❌ JSON Parse Error in {:?}: {}", path, e),
                }
            }
        }
    }

    eprintln!("⚠️ WARNING: 'sources.json' not found in any expected path. Using minimal fallback.");

    // Fallback hardcoded sources (JSON ဖိုင်မတွေ့မှသာ ဒါကိုသုံးမယ်)
    vec![
        Source { note: "Fallback".to_string(), url: "https://solana.com/docs".to_string(), topic: "solana-docs".to_string() },
        Source { note: "Fallback".to_string(), url: "https://www.anchor-lang.com/docs".to_string(), topic: "anchor-docs".to_string() },
    ]
}