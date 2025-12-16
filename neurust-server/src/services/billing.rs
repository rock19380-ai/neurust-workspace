use sqlx::PgPool;
use crate::models::UserRole;
use crate::services::ai::UsageStats;
use uuid::Uuid;

pub struct BillingService {
    pool: PgPool,
}

impl BillingService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Check if user has enough credits
    pub async fn has_sufficient_credits(&self, user_id: Uuid) -> Result<bool, sqlx::Error> {
        let rec = sqlx::query!(
            "SELECT credits, role as \"role: UserRole\" FROM users WHERE id = $1",
            user_id
        )
        .fetch_one(&self.pool)
        .await?;

        // 🔥 FREE PASS: SuperAdmin, Admin, Team don't need credits
        if matches!(rec.role, UserRole::SuperAdmin | UserRole::Admin | UserRole::Team) {
            return Ok(true); 
        }

        // Normal users need at least 5 credits
        Ok(rec.credits >= 5)
    }

    /// Deduct credits based on REAL usage (Input vs Output Split Calculation)
    pub async fn deduct_credits(
        &self,
        user_id: Uuid,
        model: &str, // Real model name from API
        action: &str,
        usage: UsageStats,
    ) -> Result<(), sqlx::Error> {
        
        // --- 💰 PRICING CONFIGURATION (Per 1 Million Tokens) ---
        // 1. Model ပေါ်မူတည်ပြီး စျေးနှုန်းခွဲခြားခြင်း
        let (input_price_per_m, output_price_per_m) = if model.contains("max") || model.contains("gpt-4") || model.contains("opus") {
            // High-End Models (Smart)
            // Example: $1.25 Input / $10.00 Output
            (1.25, 10.00) 
        } else {
            // Fast/Mini Models (Standard)
            // Example: $0.25 Input / $2.00 Output
            (0.25, 2.00) 
        };

        // 2. 🔥 SEPARATE CALCULATION (Input vs Output)
        // Formula: (Tokens / 1,000,000) * Price
        let input_cost_usd = (usage.prompt_tokens as f64 / 1_000_000.0) * input_price_per_m;
        let output_cost_usd = (usage.completion_tokens as f64 / 1_000_000.0) * output_price_per_m;

        // Total Cost (User ကုန်ကျစရိတ် အရင်း)
        let total_cost_usd = input_cost_usd + output_cost_usd;

        // 3. User Role ကို စစ်ဆေးခြင်း
        let user_role = sqlx::query!(
            "SELECT role as \"role: UserRole\" FROM users WHERE id = $1", 
            user_id
        ).fetch_one(&self.pool).await?.role;

        // 4. Determine Deduct Amount (With Profit Margin)
        // Admin/Team: 0
        // Users: (Cost * 1.2) / 0.01
        let credits_to_deduct = if matches!(user_role, UserRole::SuperAdmin | UserRole::Admin | UserRole::Team) {
            0 
        } else {
            // Profit Margin: 20% (x 1.2)
            // Credit Rate: 1 Credit = $0.01
            ((total_cost_usd * 1.2) / 0.01).ceil() as i32
        };
        
        // Ensure at least 1 credit is deducted for paying users (Micro-transaction safety)
        let final_deduction = if credits_to_deduct < 1 && !matches!(user_role, UserRole::SuperAdmin | UserRole::Admin | UserRole::Team) { 
            1 
        } else { 
            credits_to_deduct 
        };

        // 5. DB Transaction
        let mut tx = self.pool.begin().await?;

        // Only update balance if deduction > 0
        if final_deduction > 0 {
            sqlx::query!(
                "UPDATE users SET credits = credits - $1 WHERE id = $2",
                final_deduction,
                user_id
            )
            .execute(&mut *tx)
            .await?;
        }

        // 🔥 LOG EVERYTHING: Track exact USD cost for Audit
        sqlx::query!(
            "INSERT INTO usage_logs (user_id, action, model_used, input_tokens, output_tokens, cost_usd)
             VALUES ($1, $2, $3, $4, $5, $6::FLOAT8)",
            user_id,
            action,
            model,
            usage.prompt_tokens,
            usage.completion_tokens,
            total_cost_usd // Storing the Real Internal Cost
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        if final_deduction == 0 {
             println!("👑 Billing [{}]: Free usage for Team/Admin (Internal Cost: ${:.6})", model, total_cost_usd);
        } else {
             println!("💰 Billing [{}]: Cost ${:.6} (In: ${:.6}, Out: ${:.6}) -> Deducted {} credits", 
                model, total_cost_usd, input_cost_usd, output_cost_usd, final_deduction);
        }

        Ok(())
    }
}