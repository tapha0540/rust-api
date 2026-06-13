#[derive(serde::Serialize, Debug, Clone, PartialEq)]
pub struct Product {
    id: u32,
    name: String,
    description: String,
    price: f32,
    stock: u32,
    category_id: u16,
    image_url: String,
    created_at: String,
    updated_at: String,
}
