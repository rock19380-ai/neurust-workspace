use crate::prompts; 
use crate::services::knowledge_store::KnowledgeStore;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::PgPool;
use std::env;
use std::time::Duration;

const OPENROUTER_URL: &str = "https://openrouter.ai/api/v1/chat/completions";

// --- Data Structures ---

#[derive(Deserialize, Debug)]
struct ChatResponse {
    choices: Vec<Choice>,
    usage: Option<UsageStats>,
    model: String, // 🔥 Added: Capture the actual model used by API
}

#[derive(Deserialize, Debug, Clone)]
pub struct UsageStats {
    pub prompt_tokens: i32,
    pub completion_tokens: i32,
    pub total_tokens: i32,
}

#[derive(Deserialize, Debug)]
struct Choice {
    message: MessageContent,
}

#[derive(Deserialize, Debug)]
struct MessageContent {
    content: Option<String>,
}

// Service Implementation
pub struct AiService {
    client: reqwest::Client,
    api_key: String,
    knowledge_store: KnowledgeStore,
}

impl AiService {
    pub fn new(pool: PgPool) -> Self {
        let api_key = env::var("OPENROUTER_API_KEY").expect("OPENROUTER_API_KEY must be set");
        Self {
            client: reqwest::Client::builder()
                .timeout(Duration::from_secs(900)) // 15 min timeout
                .build()
                .unwrap(),
            api_key,
            knowledge_store: KnowledgeStore::new(pool),
        }
    }

    /// Smart Generation with TIERED ROUTING & BILLING SUPPORT
    /// Returns: (JSON Plan, Token Usage Stats, Used Model Name)
    pub async fn generate_project_plan(
        &self,
        user_prompt: &str,
        context: Option<String>,
    ) -> Result<(Value, UsageStats, String), String> { // 🔥 Return Tuple updated
        
        // 🔥 SMART ROUTING LOGIC
        let is_complex = user_prompt.len() > 100 
            || user_prompt.contains("create") 
            || user_prompt.contains("audit")
            || user_prompt.contains("refactor")
            || user_prompt.contains("fix")
            || context.is_some();

        let model = if is_complex {
            env::var("MODEL_THINKING").unwrap_or("openai/gpt-5.1-codex-max".to_string()) 
        } else {
            env::var("MODEL_FAST").unwrap_or("openai/gpt-5.1-codex-mini".to_string())
        };

        println!("🚀 Consulting Architect (Model: {} | Complex: {})", model, is_complex);

        // 1. RAG Search (Inject Knowledge from DB)
        let rag_content = self.knowledge_store.search(user_prompt).await;

        // 2. Construct System Message
        let system_instruction = prompts::get_architect_prompt();

        // 3. Construct Context Message
        let mut context_block = String::from("--- PROJECT CONTEXT (READ ONLY) ---\n");
        if let Some(ctx) = context {
            context_block.push_str(&ctx);
        }
        context_block.push_str("\n\n--- RAG KNOWLEDGE ---\n");
        context_block.push_str(&rag_content);

        // 4. User Request
        let user_request_block = format!(
            "--- USER REQUEST ---\n{}\n\nGenerate the JSON execution plan.", 
            user_prompt
        );

        // 5. API Payload
        let messages = vec![
            json!({ "role": "system", "content": system_instruction }),
            json!({ "role": "user", "content": context_block }), 
            json!({ "role": "user", "content": user_request_block }) 
        ];

        let max_retries = 3;

        // 🔥🔥🔥 AUTO-HEALING LOOP 🔥🔥🔥
        for attempt in 1..=max_retries {
            if attempt > 1 {
                println!("🩹 Auto-Healing Attempt {}/{}...", attempt, max_retries);
            }

            // Call API and get Usage + Model
            let (response_text, usage, used_model) = self
                .call_openrouter_with_messages(&model, messages.clone())
                .await?;

            let raw_json = self.clean_json_markdown(&response_text);
            let sanitized_json = self.sanitize_json_string(&raw_json);

            match serde_json::from_str::<Value>(&sanitized_json) {
                Ok(plan) => {
                    // ✅ SUCCESS: Return Plan, Usage AND Real Model Name
                    return Ok((plan, usage, used_model));
                },
                Err(e) => {
                    println!("❌ JSON Error on attempt {}: {}", attempt, e);
                    if attempt == max_retries {
                        return Err(format!("Failed to parse JSON. Error: {}", e));
                    }
                }
            }
        }

        Err("Auto-healing failed.".to_string())
    }

    /// Audit Code -> Returns (Report, Usage, Model)
    pub async fn audit_code(&self, code: &str) -> Result<(String, UsageStats, String), String> {
        let model = env::var("MODEL_THINKING").unwrap_or("openai/gpt-5.1-codex-max".to_string());
        println!("🕵️ Auditing with Model: {}", model);
        
        let messages = vec![
            json!({ "role": "system", "content": prompts::SYSTEM_AUDITOR }),
            json!({ "role": "user", "content": code })
        ];

        self.call_openrouter_with_messages(&model, messages).await
    }

    // Unified API Call Function (Now returns UsageStats AND Model Name)
    async fn call_openrouter_with_messages(
        &self,
        model: &str,
        messages: Vec<Value>,
    ) -> Result<(String, UsageStats, String), String> { // 🔥 Return Tuple updated
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        let mut auth_val = HeaderValue::from_str(&format!("Bearer {}", self.api_key)).unwrap();
        auth_val.set_sensitive(true);
        headers.insert(AUTHORIZATION, auth_val);
        
        headers.insert("HTTP-Referer", HeaderValue::from_static("https://neurust.app"));
        headers.insert("X-Title", HeaderValue::from_static("Neurust AI"));

        let payload = json!({
            "model": model,
            "messages": messages,
            "temperature": 0.2,
            "max_tokens": 16000, 
        });

        let res = self.client.post(OPENROUTER_URL)
            .headers(headers)
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("Request Failed: {}", e))?;

        let status = res.status();
        if !status.is_success() {
            let error_text = res.text().await.unwrap_or_default();
            return Err(format!("API Error {}: {}", status, error_text));
        }

        let body: ChatResponse = res.json().await.map_err(|e| format!("JSON Parse Error: {}", e))?;
        
        let content = body.choices.first()
            .ok_or("No response choices")?
            .message.content.clone()
            .ok_or("AI returned no content".to_string())?;

        // Capture Usage
        let usage = body.usage.unwrap_or(UsageStats { 
            prompt_tokens: 0, 
            completion_tokens: 0, 
            total_tokens: 0 
        });

        // 🔥 Return Content, Usage, AND Actual Model Name
        Ok((content, usage, body.model))
    }

    fn clean_json_markdown(&self, input: &str) -> String {
        input.trim().trim_start_matches("```json").trim_start_matches("```").trim_end_matches("```").trim().to_string()
    }
    
    fn sanitize_json_string(&self, input: &str) -> String {
        let mut output = String::with_capacity(input.len());
        let mut in_string = false;
        let mut escaped = false;
        for c in input.chars() {
             match c {
                '"' => { if !escaped { in_string = !in_string; } output.push(c); escaped = false; }
                '\\' => { output.push(c); escaped = !escaped; }
                '\n' | '\r' => { if in_string { output.push_str("\\n"); } else { output.push(c); } escaped = false; }
                '\t' => { if in_string { output.push_str("\\t"); } else { output.push(c); } escaped = false; }
                _ => { output.push(c); escaped = false; }
            }
        }
        output
    }
}