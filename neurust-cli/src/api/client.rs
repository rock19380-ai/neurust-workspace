use reqwest::{Client, StatusCode}; // 🔥 Added StatusCode import
use serde::Deserialize;
use serde_json::json;
use anyhow::{Result, anyhow, Context};
use std::time::Duration;

#[derive(Clone)]
pub struct ApiClient {
    base_url: String,
    client: Client,
}

#[derive(Debug, Deserialize)]
pub struct DeviceFlowInit {
    pub device_code: String,
    pub user_code: String,
    pub verification_uri: String,
    pub interval: u64,
}

#[derive(Debug, Deserialize)]
pub struct PollResponse {
    pub status: String, // "pending", "verified", "expired"
    pub token: Option<String>,
    pub message: Option<String>,
}

impl ApiClient {
    pub fn new(base_url: String) -> Self {
        // Defensive: URL Cleaning
        let mut clean_url = base_url.trim().to_string();
        if clean_url.starts_with('[') && clean_url.contains("](") {
             if let Some(start) = clean_url.find("](") {
                 if let Some(end) = clean_url[start..].find(')') {
                     clean_url = clean_url[start+2 .. start+end].to_string();
                 }
             }
        }
        if !clean_url.starts_with("http") {
            clean_url = format!("http://{}", clean_url);
        }
        if clean_url.ends_with('/') {
            clean_url.pop();
        }

        Self {
            base_url: clean_url,
            client: Client::builder()
                .timeout(Duration::from_secs(600)) // 10 mins timeout
                .build()
                .unwrap(),
        }
    }

    /// Existing Method: Fetch AI Plan
    pub async fn fetch_plan(&self, prompt: &str, context: Option<String>) -> Result<serde_json::Value> {
        let url = format!("{}/api/agent/plan", self.base_url);
        let payload = json!({ "prompt": prompt, "context": context });

        let response = self.client.post(&url)
            .json(&payload)
            .send()
            .await
            .map_err(|e| anyhow!("Failed to connect to Brain: {}", e))?;

        let status = response.status();

        if !status.is_success() {
            let err = response.text().await.unwrap_or_default();
            return Err(anyhow!("Server Error ({}): {}", status, err));
        }

        Ok(response.json().await?)
    }

    /// Existing Method: Audit Code
    pub async fn audit_code(&self, code_payload: &str) -> Result<String> {
        let url = format!("{}/api/agent/audit", self.base_url);
        let payload = json!({ "code": code_payload });

        let response = self.client.post(&url)
            .json(&payload)
            .send()
            .await?;

        let status = response.status();

        if !status.is_success() {
            return Err(anyhow!("Audit Failed: {}", status));
        }

        let body: serde_json::Value = response.json().await?;
        body["report"].as_str().map(|s| s.to_string()).ok_or(anyhow!("No report found"))
    }

    /// Existing Method: Scraper
    pub async fn scrape_url(&self, target_url: &str) -> Result<String> {
        let url = format!("{}/api/agent/browse", self.base_url);
        let payload = json!({ "url": target_url });
        let res = self.client.post(&url).json(&payload).send().await?;
        
        let status = res.status();
        if !status.is_success() { return Err(anyhow!("Scraper Error")); }
        
        let body: serde_json::Value = res.json().await?;
        body["content"].as_str().map(|s| s.to_string()).ok_or(anyhow!("No content"))
    }

    // 🔥🔥🔥 DEVICE FLOW METHODS 🔥🔥🔥

    /// Step 1: Initiate Login Flow
    pub async fn initiate_device_flow(&self) -> Result<DeviceFlowInit> {
        let url = format!("{}/api/auth/device/initiate", self.base_url);
        
        // 🔥 CRITICAL FIX: Send empty JSON to force `Content-Type: application/json` header
        // Axum server requires this header for Json<> extractor.
        let response = self.client.post(&url)
            .json(&json!({})) 
            .send()
            .await
            .context("Failed to connect to Auth Server")?;

        let status = response.status();

        if !status.is_success() {
            let err = response.text().await.unwrap_or_default();
            return Err(anyhow!("Auth Init Failed ({}): {}", status, err));
        }

        let data: DeviceFlowInit = response.json().await
            .context("Invalid JSON from Auth Init")?;
            
        Ok(data)
    }

    /// Step 2: Poll for Status
    pub async fn poll_device_flow(&self, device_code: &str) -> Result<PollResponse> {
        let url = format!("{}/api/auth/device/poll", self.base_url);
        let payload = json!({ "device_code": device_code });

        let response = self.client.post(&url)
            .json(&payload)
            .send()
            .await?;

        // 🔥 FIX: Capture status code FIRST before consuming body
        let status = response.status();

        // 404 means expired/invalid, which is a valid state for polling
        if status == StatusCode::NOT_FOUND {
             return Ok(PollResponse { status: "expired".to_string(), token: None, message: None });
        }

        if !status.is_success() {
             // Try to parse error message from JSON, fallback to empty
             let body: serde_json::Value = response.json().await.unwrap_or(json!({}));
             
             // If status is pending (rare case for error code but possible), return it safely
             if let Some(status_val) = body["status"].as_str() {
                 return Ok(PollResponse { 
                     status: status_val.to_string(), 
                     token: None, 
                     message: body["message"].as_str().map(|s| s.to_string()) 
                 });
             }
             
             return Err(anyhow!("Poll Error: {}", status));
        }

        let data: PollResponse = response.json().await?;
        Ok(data)
    }
}