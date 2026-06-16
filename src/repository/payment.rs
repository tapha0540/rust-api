use sqlx::{MySql, Pool, QueryBuilder, mysql::MySqlQueryResult, query, query_as};

use crate::models::payment::{Payment, PaymentMethod, PaymentStatus};

pub struct PaymentRepository;

impl PaymentRepository {
    pub async fn insert(
        pool: &Pool<MySql>,
        order_id: i32,
        amount: f32,
        method: PaymentMethod,
        status: PaymentStatus,
    ) -> Result<MySqlQueryResult, sqlx::Error> {
        query("INSERT INTO payments(order_id, amount, method, status) VALUES (?,?,?,?)")
            .bind(order_id)
            .bind(amount)
            .bind(method.as_str())
            .bind(status.as_str())
            .execute(pool)
            .await
    }

    pub async fn find_by_id(pool: &Pool<MySql>, id: i32) -> Result<Payment, sqlx::Error> {
        query_as::<_, Payment>(
            "SELECT id, order_id, amount, method, status, created_at FROM payments WHERE id = ?",
        )
        .bind(id)
        .fetch_one(pool)
        .await
    }

    pub async fn find_all(pool: &Pool<MySql>) -> Result<Vec<Payment>, sqlx::Error> {
        query_as::<_, Payment>(
            "SELECT id, order_id, amount, method, status, created_at FROM payments",
        )
        .fetch_all(pool)
        .await
    }

    pub async fn delete(pool: &Pool<MySql>, id: i32) -> Result<MySqlQueryResult, sqlx::Error> {
        query("DELETE FROM payments WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await
    }
    pub async fn update(
        pool: &Pool<MySql>,
        payment: Payment,
        id: i32,
    ) -> Result<MySqlQueryResult, sqlx::Error> {
        let mut query_builder = QueryBuilder::<MySql>::new("UPDATE payments SET ");

        let mut separated = query_builder.separated(", ");
        let mut has_fields = false;

        if let Some(order_id) = payment.order_id {
            separated.push("order_id = ").push_bind(order_id);
            has_fields = true;
        }

        if let Some(amount) = payment.amount {
            separated.push("amount = ").push_bind(amount);
            has_fields = true;
        }
        if let Some(method) = payment.method {
            separated.push("method = ").push_bind(method);
            has_fields = true;
        }
        if let Some(status) = payment.status {
            separated.push("status = ").push_bind(status);
            has_fields = true;
        }

        drop(separated);

        if !has_fields {
            return Err(sqlx::Error::Protocol(
                "Aucun champ fourni pour la mise à jour".into(),
            ));
        }

        query_builder.push(" WHERE id = ").push_bind(id);
        let query = query_builder.build();
        query.execute(pool).await
    }
}
