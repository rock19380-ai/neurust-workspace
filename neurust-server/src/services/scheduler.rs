use crate::services::knowledge_store::KnowledgeStore;
use crate::services::scraper::ScraperService;
use crate::sources; // 🔥 Reuse the existing sources module
use sqlx::PgPool;
use std::time::Duration;
use tokio::time;

pub struct UpdateScheduler;

impl UpdateScheduler {
    pub async fn start_weekly_updates(pool: PgPool) {
        tokio::spawn(async move {
            let scraper = ScraperService::new(pool.clone());
            let store = KnowledgeStore::new(pool.clone());

            loop {
                println!("⏰ Starting Weekly Knowledge Update...");

                // ၁. Sources တွေကို Central Module ကနေ ယူမယ် (Code duplication မဖြစ်အောင်)
                let sources = sources::get_trusted_sources();

                if sources.is_empty() {
                    println!("⚠️ No sources found in 'data/sources.json'. Using defaults/fallback.");
                } else {
                    println!("📚 Found {} sources to process.", sources.len());
                }

                // ၂. တစ်ခုချင်းစီ လိုက်ဖတ်ပြီး Update လုပ်မယ်
                for source in &sources {
                    println!("🔄 Processing Topic: {} ({})", source.topic, source.url);

                    // ScraperService ကို ခေါ်ပြီး URL ကို ဖတ်မယ်၊ DB ထဲထည့်မယ်
                    match scraper.scrape_and_save(&source.url, &source.topic).await {
                        Ok(_) => println!("✅ Updated Successfully: {}", source.topic),
                        Err(e) => eprintln!("❌ Failed to update {}: {}", source.url, e),
                    }

                    // Server ဝန်မပိအောင် 2 စက္ကန့် နားမယ် (Rate Limiting)
                    time::sleep(Duration::from_secs(2)).await;
                }

                // ၃. Pruning (JSON ထဲမှာ မရှိတော့တဲ့ အဟောင်းတွေကို ရှင်းမယ်)
                println!("🧹 Pruning stale data...");
                
                // 🔥 FIX: sources::Source type ကို သုံးထားလို့ Type mismatch မဖြစ်တော့ပါ
                if let Err(e) = store.prune_stale_data(&sources).await {
                    eprintln!("❌ Pruning failed: {}", e);
                } else {
                    println!("✅ Knowledge Base cleanup complete.");
                }

                println!("💤 Update cycle finished. Sleeping for 7 days...");
                // ၄. နောက်ထပ် ၁ ပတ် ကြာမှ ပြန်လုပ်မယ်
                time::sleep(Duration::from_secs(60 * 60 * 24 * 7)).await;
            }
        });
    }
}