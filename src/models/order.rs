
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq,)]
pub enum OrderStatus {
    Pending,
    Processing,
    Shipped,
    Delivered,
    Cancelled,
}

impl OrderStatus {
    pub fn as_str(&self) -> String {
        match self {
            Self::Cancelled => "cancelled".to_string(),
            Self::Delivered => "delivered".to_string(),
            Self::Pending => "pending".to_string(),
            Self::Processing => "processing".to_string(),
            Self::Shipped => "shipped".to_string(),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, sqlx::FromRow)]
pub struct Order {
    id: u32,
    user_id: u32,
    status: OrderStatus,
    total: f32,
    created_at: String,
    updated_at: String,
}
