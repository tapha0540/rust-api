#[derive(serde::Serialize, Debug, Clone, PartialEq)]
pub struct Review {
    id: u32,
    product_id: u32,
    user_id: u32,
    rating: u8,
    comment: String,
    created_at: String,
    updated_at: String,
}
