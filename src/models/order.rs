use chrono::{DateTime, Utc};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum OrderStatus {
    Pending,
    Processing,
    Shipped,
    Delivered,
    Cancelled,
}

impl OrderStatus {
    pub fn new(status: String) -> Option<Self> {
        match status.as_str() {
            "cancelled" => Some(Self::Cancelled),
            "delivered" => Some(Self::Delivered),
            "pending" => Some(Self::Pending),
            "processing" => Some(Self::Processing),
            "shipped" => Some(Self::Shipped),
            _ => {
                dbg!("Invalid status value !");
                None
            }
        }
    }
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
    pub id: Option<i32>,
    pub user_id: Option<i32>,
    pub status: Option<String>,
    pub total: Option<f32>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl Order {
    pub fn get_status(&self) -> Option<OrderStatus> {
        OrderStatus::new(self.status.clone()?)
    }
}
