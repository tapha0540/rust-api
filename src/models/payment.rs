use chrono::{DateTime, Utc};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum PaymentMethod {
    Wave,
    OrangeMoney,
    Kpay,
    Cash,
}

impl PaymentMethod {
    pub fn new(method: String) -> Option<Self> {
        match method.as_str() {
            "wave" => Some(Self::Wave),
            "orange money" => Some(Self::OrangeMoney),
            "kpay" => Some(Self::Kpay),
            "cash" => Some(Self::Cash),
            _ => {
                dbg!("Invalid method value !");
                None
            }
        }
    }
    pub fn as_str(&self) -> String {
        match self {
            Self::Wave => "wave".to_string(),
            Self::OrangeMoney => "orange money".to_string(),
            Self::Kpay => "kpay".to_string(),
            Self::Cash => "cash".to_string(),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum PaymentStatus {
    Pending,
    Completed,
    Failed,
}

impl PaymentStatus {
    pub fn new(status: String) -> Option<Self> {
        match status.as_str() {
            "pending" => Some(Self::Pending),
            "completed" => Some(Self::Completed),
            "Failed" => Some(Self::Failed),
            _ => {
                dbg!("Invalid PaymentStatus enum value");
                None
            }
        }
    }
    pub fn as_str(&self) -> String {
        match self {
            Self::Pending => "pending".to_string(),
            Self::Completed => "completed".to_string(),
            Self::Failed => "failed".to_string(),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, sqlx::FromRow)]
pub struct Payment {
    pub id: Option<i32>,
    pub order_id: Option<i32>,
    pub amount: Option<f32>,
    pub method: Option<String>,
    pub status: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
}

impl Payment {
    pub fn get_method(&self) -> Option<PaymentMethod> {
        PaymentMethod::new(self.method.clone()?)
    }
    pub fn get_status(&self) -> Option<PaymentStatus> {
        PaymentStatus::new(self.status.clone()?)
    }
}
