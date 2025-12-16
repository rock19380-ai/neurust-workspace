use chrono::{DateTime, Utc};
use sqlx::PgPool;
use crate::sources; // 🔥 Import sources module for pruning

#[derive(sqlx::FromRow, Debug)]
pub struct DocEntry {
    pub id: i64,
    pub source_url: String, // ✅ Renamed to match DB
    pub topic: String,
    pub content: String,
    pub last_scraped_at: DateTime<Utc>, // ✅ Renamed to match DB
}

#[derive(Clone)]
pub struct KnowledgeStore {
    pool: PgPool,
}

impl KnowledgeStore {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn save_doc(&self, url: &str, topic: &str, content: &str) -> Result<(), sqlx::Error> {
        // 🔥 Updated SQL to match new schema
        sqlx::query(
            r#"
            INSERT INTO knowledge_base (source_url, topic, content, last_scraped_at)
            VALUES ($1, $2, $3, NOW())
            ON CONFLICT (source_url) 
            DO UPDATE SET 
                content = EXCLUDED.content,
                topic = EXCLUDED.topic,
                last_scraped_at = NOW()
            "#,
        )
        .bind(url)
        .bind(topic)
        .bind(content)
        .execute(&self.pool)
        .await?;

        println!("💾 Saved to DB: {}", url);
        Ok(())
    }

    // 🔥 POWERFUL SEARCH ENGINE (Hybrid: FTS + Fallback)
    pub async fn search(&self, query: &str) -> String {
        // 1. PostgreSQL Full Text Search (FTS)
        let rows = sqlx::query_as::<_, DocEntry>(
            r#"
            SELECT id, source_url, topic, content, last_scraped_at
            FROM knowledge_base
            WHERE to_tsvector('english', topic || ' ' || content) @@ websearch_to_tsquery('english', $1)
            ORDER BY ts_rank(to_tsvector('english', topic || ' ' || content), websearch_to_tsquery('english', $1)) DESC
            LIMIT 3
            "#
        )
        .bind(query)
        .fetch_all(&self.pool)
        .await
        .unwrap_or_default();

        let mut results = String::new();
        for row in rows {
            let snippet: String = row.content.chars().take(2000).collect();
            results.push_str(&format!(
                "\n--- SOURCE: {} ({}) ---\n{}\n",
                row.source_url, row.topic, snippet
            ));
        }

        // 2. Fallback if FTS fails
        if results.is_empty() {
            println!("⚠️ FTS returned empty, trying fallback search...");
            self.fallback_search(query).await
        } else {
            format!("RELEVANT DOCUMENTATION FOUND:\n{}", results)
        }
    }

    async fn fallback_search(&self, query: &str) -> String {
        let pattern = format!("%{}%", query);
        let rows = sqlx::query_as::<_, DocEntry>(
            r#"
            SELECT id, source_url, topic, content, last_scraped_at
            FROM knowledge_base 
            WHERE content ILIKE $1 OR topic ILIKE $1
            ORDER BY last_scraped_at DESC
            LIMIT 2
            "#,
        )
        .bind(pattern)
        .fetch_all(&self.pool)
        .await
        .unwrap_or_default();

        let mut results = String::new();
        for row in rows {
            let snippet: String = row.content.chars().take(1500).collect();
            results.push_str(&format!(
                "\n--- SOURCE (Fallback): {} ---\n{}\n",
                row.topic, snippet
            ));
        }

        if results.is_empty() {
            String::new()
        } else {
            format!("RELEVANT DOCUMENTATION FOUND (FALLBACK):\n{}", results)
        }
    }

    // 🔥 Prune data that is no longer in sources.json
    pub async fn prune_stale_data(
        &self,
        active_sources: &[sources::Source], // Using crate::sources::Source
    ) -> Result<(), sqlx::Error> {
        let active_urls: Vec<String> = active_sources.iter().map(|s| s.url.clone()).collect();
        if active_urls.is_empty() {
            return Ok(());
        }

        let result = sqlx::query("DELETE FROM knowledge_base WHERE source_url <> ALL($1)")
            .bind(&active_urls)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() > 0 {
            println!("🗑️ Pruned {} obsolete documents.", result.rows_affected());
        }
        Ok(())
    }
}