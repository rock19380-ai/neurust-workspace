use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::env;

// Database Connection Pool ကို တည်ဆောက်မယ့် Function
pub async fn init_db() -> Pool<Postgres> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");

    println!("🔌 Connecting to Database...");

    PgPoolOptions::new()
        .max_connections(5) // Laptop မလေးအောင် connection နည်းနည်းပဲယူမယ်
        .connect(&database_url)
        .await
        .expect("Failed to connect to Postgres")
}

// ... အပေါ်က code တွေ အတူတူပဲ ...

#[derive(serde::Serialize)]
pub struct User {
    pub id: i32,
    pub wallet_address: String,
    pub is_pro: bool,
}

/// User ကို ရှာပါ၊ မရှိရင် အသစ်ဖန်တီးပါ (Get or Create)
pub async fn get_or_create_user(pool: &Pool<Postgres>, wallet: &str) -> Result<User, sqlx::Error> {
    // ၁. ရှိပြီးသားလား စစ်မယ် (SELECT)
    // ပြင်ဆင်ချက်: is_pro နေရာမှာ `is_pro as "is_pro!"` လို့ ပြင်ပါ
    let existing_user = sqlx::query_as!(
        User,
        r#"
        SELECT id, wallet_address, is_pro as "is_pro!" 
        FROM users 
        WHERE wallet_address = $1
        "#,
        wallet
    )
    .fetch_optional(pool)
    .await?;

    if let Some(user) = existing_user {
        return Ok(user);
    }

    // ၂. မရှိရင် အသစ်ထည့်မယ် (INSERT)
    // ပြင်ဆင်ချက်: is_pro နေရာမှာ `is_pro as "is_pro!"` လို့ ပြင်ပါ
    let new_user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (wallet_address) 
        VALUES ($1) 
        RETURNING id, wallet_address, is_pro as "is_pro!"
        "#,
        wallet
    )
    .fetch_one(pool)
    .await?;

    Ok(new_user)
}
