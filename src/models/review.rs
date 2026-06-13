use chrono::{DateTime, Utc};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, sqlx::FromRow)]
pub struct Review {
    id: u32,
    product_id: u32,
    user_id: u32,
    rating: u8,
    comment: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
