#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Category {
    pub id: u16,
    pub name: String,
    pub description: String,
    pub parent_id: u16,
    pub icon_url: String,
    pub created_at: String,
    pub updated_at: String,
}
