#[derive(serde::Serialize, Debug, Clone, Copy, PartialEq, Eq)]
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
#[derive(serde::Serialize, Debug, Clone, PartialEq, Eq)]
pub struct User {
    id: u32,
    first_name: String,
    last_name: String,
    email: String,
    password: String,
    role: UserRole,
    phone: String,
    profile_url: String,
    created_at: String,
    updated_at: String,
}
