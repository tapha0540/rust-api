#[derive(serde::Serialize, Debug, Clone, PartialEq)]
pub enum PaymentMethod {
    Wave,
    OrangeMoney,
    Kpay,
    Cash,
}

impl PaymentMethod {
    pub fn as_str(&self) -> String {
        match self {
            Self::Wave => "wave".to_string(),
            Self::OrangeMoney => "orange money".to_string(),
            Self::Kpay => "kpay".to_string(),
            Self::Cash => "cash".to_string(),
        }
    }
}

#[derive(serde::Serialize, Debug, Clone, PartialEq)]
pub enum PaymentStatus {
    Pending,
    Completed,
    Failed,
}

impl PaymentStatus {
    pub fn as_str(&self) -> String {
        match self {
            Self::Pending => "pending".to_string(),
            Self::Completed => "completed".to_string(),
            Self::Failed => "failed".to_string(),
        }
    }
}

#[derive(serde::Serialize, Debug, Clone, PartialEq)]
pub struct Payment {
    id: u32,
    order_id: u32,
    amount: f32,
    method: PaymentMethod,
    status: PaymentStatus,
    created_at: String,
}
