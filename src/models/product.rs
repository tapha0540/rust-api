use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, sqlx::FromRow)]
pub struct Product {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub price: Option<Decimal>,
    pub stock: Option<i32>,
    pub category_id: Option<i16>,
    pub image_url: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}
