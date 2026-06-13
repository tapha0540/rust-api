
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, sqlx::FromRow)]

pub struct OrderItem {
    id: u16,
    order_id: u32,
    product_id: u32,
    quantity: u16,
    price: f32,
}