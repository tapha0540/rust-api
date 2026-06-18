use chrono::{DateTime, Utc};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum UserRole {
    Customer,
    Admin,
}

impl UserRole {
    pub fn new(role: String) -> Option<Self> {
        match role.as_str() {
            "admin" => Some(Self::Admin),
            "customer" => Some(Self::Customer),
            _ => {
                tracing::error!("Invalide role value");
                None
            }
        }
    }
    pub fn as_str(&self) -> String {
        match self {
            Self::Customer => "customer".to_string(),
            Self::Admin => "admin".to_string(),
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, sqlx::FromRow)]
pub struct User {
    pub id: Option<i32>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub role: Option<String>,
    pub phone: Option<String>,
    pub profile_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn get_role(&self) -> Option<UserRole> {
        UserRole::new(self.role.clone()?)
    }
}
