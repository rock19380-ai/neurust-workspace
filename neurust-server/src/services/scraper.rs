use crate::services::knowledge_store::KnowledgeStore;
use reqwest::Client;
use sqlx::PgPool;
use std::time::Duration;

pub struct ScraperService {
    client: Client,
    store: KnowledgeStore,
}

impl ScraperService {
    // Constructor
    pub fn new(pool: PgPool) -> Self {
        Self {
            client: Client::builder()
                .timeout(Duration::from_secs(10)) // 10s Timeout
                .user_agent("Neurust-Agent/1.0") // Fake User Agent
                .build()
                .unwrap(),
            store: KnowledgeStore::new(pool),
        }
    }

    // URL ကိုဖတ်ပြီး Clean Text ပြောင်းပေးမည့် Function
    pub async fn scrape_url(&self, url: &str) -> Result<String, String> {
        println!("🕷️ Scraping URL: {}", url);

        let response = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|e| format!("Failed to connect: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("Error: HTTP {}", response.status()));
        }

        let html_content = response
            .text()
            .await
            .map_err(|e| format!("Failed to read text: {}", e))?;

        // 🔥 HTML to Clean Text (Using html2text crate)
        // width 80 characters နဲ့ စာစီပေးမယ်
        let clean_text = html2text::from_read(html_content.as_bytes(), 80);

        // စာအရမ်းရှည်ရင် AI Token ပြည့်သွားနိုင်လို့ အလုံးရေ ၈၀၀၀ လောက်ပဲ ယူမယ်
        let truncated_text: String = clean_text.chars().take(8000).collect();

        Ok(truncated_text)
    }

    // URL ကိုဖတ်မယ်၊ ပြီးရင် Database ထဲသိမ်းမယ်
    pub async fn scrape_and_save(&self, url: &str, topic: &str) -> Result<String, String> {
        // ၁. scrape_url logic ကို လှမ်းခေါ်သုံးမယ်
        let content = self.scrape_url(url).await?;

        // ၂. ရလာတဲ့ Content ကို Database ထဲ Upsert လုပ်မယ်
        // (KnowledgeStore.save_doc က DB အသစ်နဲ့ ချိတ်ပြီးသားပါ)
        if let Err(e) = self.store.save_doc(url, topic, &content).await {
            eprintln!("❌ Failed to save to DB: {}", e);
            return Err(format!("Database Error: {}", e));
        }

        println!("💾 Knowledge stored for topic: {}", topic);
        Ok(content)
    }
}