use chrono::{DateTime, Utc};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum UserRole {
    Customer,
    Admin,
}

impl UserRole {
    pub fn as_str(&self) -> String {
        match self {
            Self::Customer => "customer".to_string(),
            Self::Admin => "admin".to_string(),
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, sqlx::FromRow)]
pub struct User {
    id: i32,
    first_name: String,
    last_name: String,
    email: String,
    password: String,
    role: UserRole,
    phone: String,
    profile_url: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
