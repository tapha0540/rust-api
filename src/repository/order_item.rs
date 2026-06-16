use sqlx::{MySql, Pool, QueryBuilder, mysql::MySqlQueryResult, query, query_as};

use crate::models::order_item::OrderItem;

pub struct OrderItemRepository;

impl OrderItemRepository {
    pub async fn insert(
        pool: &Pool<MySql>,
        order_id: i32,
        product_id: i32,
        quantity: i16,
        price: f32,
    ) -> Result<MySqlQueryResult, sqlx::Error> {
        query("INSERT INTO order_items(order_id, product_id, quantity, price) VALUES (?,?,?,?)")
            .bind(order_id)
            .bind(product_id)
            .bind(quantity)
            .bind(price)
            .execute(pool)
            .await
    }

    pub async fn find_by_id(pool: &Pool<MySql>, id: i32) -> Result<OrderItem, sqlx::Error> {
        query_as::<MySql, OrderItem>(
            "SELECT id, order_id, product_id, quantity, price FROM order_items WHERE id = ?",
        )
        .bind(id)
        .fetch_one(pool)
        .await
    }
    pub async fn find_all(pool: &Pool<MySql>) -> Result<Vec<OrderItem>, sqlx::Error> {
        query_as::<MySql, OrderItem>(
            "SELECT id, order_id, product_id, quantity, price FROM order_items",
        )
        .fetch_all(pool)
        .await
    }

    pub async fn delete(pool: &Pool<MySql>, id: i32) -> Result<MySqlQueryResult, sqlx::Error> {
        query("DELETE FROM order_items WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await
    }
    pub async fn update(
        pool: &Pool<MySql>,
        order_item: OrderItem,
        id: i32,
    ) -> Result<MySqlQueryResult, sqlx::Error> {
        let mut query_builder = QueryBuilder::new("UPDATE order_items SET ");
        let mut separated = query_builder.separated(", ");
        let mut has_fields = false;

        if let Some(order_id) = order_item.order_id {
            separated.push("order_id = ").push_bind(order_id);
            has_fields = true;
        }
        if let Some(product_id) = order_item.product_id {
            separated.push("product_id = ").push_bind(product_id);
            has_fields = true;
        }
        if let Some(quantity) = order_item.quantity {
            separated.push("quantity = ").push_bind(quantity);
            has_fields = true;
        }
        if let Some(price) = order_item.price {
            separated.push("price = ").push_bind(price);
            has_fields = true;
        }

        drop(separated);

        if !has_fields {
            return Err(sqlx::Error::Protocol("Aucun champ fourni pour la mise à jour".into()));
        }
        query_builder.push("WHERE id = ").push_bind(id);
        let query = query_builder.build();
        query.execute(pool).await
    }
}
