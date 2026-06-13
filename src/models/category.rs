
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, sqlx::FromRow)]
pub struct Category {
    pub id: i32,
    pub name: Option<String>,
    pub description: Option<String>,
    pub parent_id: Option<i32>,
    pub icon_url: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}
