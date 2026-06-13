use chrono::{DateTime, Utc};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, sqlx::FromRow)]
pub struct Product {
    id: u32,
    name: String,
    description: String,
    price: f32,
    stock: u32,
    category_id: u16,
    image_url: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
