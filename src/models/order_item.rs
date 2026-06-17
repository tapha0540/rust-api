use rust_decimal::Decimal;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, sqlx::FromRow)]

pub struct OrderItem {
    pub id: Option<i16>,
    pub order_id: Option<i32>,
    pub product_id: Option<i32>,
    pub quantity: Option<i16>,
    pub price: Option<Decimal>,
}