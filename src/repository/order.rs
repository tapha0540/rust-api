use axum::http::status;
use sqlx::{MySql, Pool, QueryBuilder, mysql::MySqlQueryResult, query, query_as, query_builder};

use crate::models::order::{Order, OrderStatus};

pub struct OrderRepository;

impl OrderRepository {
    pub async fn insert(
        pool: &Pool<MySql>,
        user_id: i32,
        status: OrderStatus,
        total: f32,
    ) -> Result<MySqlQueryResult, sqlx::Error> {
        query("INSERT INTO orders(user_id, status, total) VALUES (?,?,?)")
            .bind(user_id)
            .bind(status.as_str())
            .bind(total)
            .execute(pool)
            .await
    }
    pub async fn find_order_by_id(pool: &Pool<MySql>, id: i32) -> Result<Order, sqlx::Error> {
        query_as::<MySql, Order>(
            "SELECT id, user_id, status, total, created_at, updated_at FROM orders WHERE id = ?",
        )
        .fetch_one(pool)
        .await
    }
    pub async fn find_orders(pool: &Pool<MySql>) -> Result<Vec<Order>, sqlx::Error> {
        query_as::<MySql, Order>(
            "SELECT id, user_id, status, total, created_at, updated_at FROM orders",
        )
        .fetch_all(pool)
        .await
    }
    pub async fn delete(pool: &Pool<MySql>, id: i32) -> Result<MySqlQueryResult, sqlx::Error> {
        query("DELETE FROM orders WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await
    }
    pub async fn update(
        pool: &Pool<MySql>,
        order: Order,
        id: i32,
    ) -> Result<MySqlQueryResult, sqlx::Error> {
        let mut query_builder = QueryBuilder::new("UPDATE orders SET ");

        let mut separated = query_builder.separated(", ");
        let mut has_fields = false;

        if let Some(user_id) = order.user_id {
            separated.push("user_id = ").push_bind(user_id);
            has_fields = true;
        }
        if let Some(status) = order.status {
            separated.push("status = ").push_bind(status);
            has_fields = true;
        }
        if let Some(total) = order.total {
            separated.push("total = ").push_bind(total);
            has_fields = true;
        }

        if !has_fields {
            return Err(sqlx::Error::Protocol(
                "Aucun champ fourni pour la mise à jour".into(),
            ));
        }

        drop(separated);
        query_builder.push("WHERE id = ").push_bind(id);
        let query = query_builder.build();
        query.execute(pool).await
    }
}
