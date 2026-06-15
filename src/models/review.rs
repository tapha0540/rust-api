use chrono::{DateTime, Utc};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, sqlx::FromRow)]
pub struct Review {
    pub id: Option<i32>,
    pub product_id: Option<i32>,
    pub user_id: Option<i32>,
    pub rating: Option<i8>,
    pub comment: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}
