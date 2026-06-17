use sqlx::{MySql, Pool, mysql::MySqlQueryResult, query, query_as};

use crate::models::review::Review;

pub struct ReviewRepository;

impl ReviewRepository {
    pub async fn insert(pool: &Pool<MySql>) -> Result<MySqlQueryResult, sqlx::Error> {
        query("INSERT INTO reviews(product_id, user_id, rating, comment) VALUES (?, ?, ?, ?)")
            .execute(pool)
            .await
    }
    pub async fn find_by_ids(pool: &Pool<MySql>, id: i32) -> Result<Review, sqlx::Error> {
        query_as::<_, Review>("SELECT id, product_id, user_id, rating, comment, created_at, updated_at FROM reviews WHERE id = ?")
        .bind(id)
        .fetch_one(pool)
        .await
    }
    pub async fn find_all(pool: &Pool<MySql>) -> Result<Vec<Review>, sqlx::Error> {
        query_as::<_, Review>("SELECT id, product_id, user_id, rating, comment, created_at, updated_at FROM reviews WHERE id = ?")
        .fetch_all(pool)
        .await
    }
    pub async fn delete(pool: &Pool<MySql>, id: i32) -> Result<MySqlQueryResult, sqlx::Error> {
        query("DELETE FROM reviews WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await
    }
}
