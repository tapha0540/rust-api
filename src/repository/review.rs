use sqlx::{MySql, Pool, QueryBuilder, mysql::MySqlQueryResult, query, query_as};

use crate::models::{review::Review, user};

pub struct ReviewRepository;

impl ReviewRepository {
    pub async fn insert(
        pool: &Pool<MySql>,
        product_id: i32,
        user_id: i32,
        rating: i8,
        comment: String,
    ) -> Result<MySqlQueryResult, sqlx::Error> {
        query("INSERT INTO reviews(product_id, user_id, rating, comment) VALUES (?, ?, ?, ?)")
            .bind(product_id)
            .bind(user_id)
            .bind(rating)
            .bind(comment)
            .execute(pool)
            .await
    }
    pub async fn find_by_id(pool: &Pool<MySql>, id: i32) -> Result<Review, sqlx::Error> {
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
    pub async fn update(
        pool: &Pool<MySql>,
        review: Review,
        id: i32,
    ) -> Result<MySqlQueryResult, sqlx::Error> {
        let mut query_builder = QueryBuilder::<MySql>::new("UPDATE reviews SET");

        let mut separated = query_builder.separated(", ");
        let mut has_fields = false;

        if let Some(product_id) = review.product_id {
            separated
                .push("product_id = ")
                .push_bind_unseparated(product_id);
            has_fields = true;
        }
        if let Some(user_id) = review.user_id {
            separated.push("user_id = ").push_bind_unseparated(user_id);
            has_fields = true;
        }

        if let Some(rating) = review.rating {
            separated.push("rating = ").push_bind_unseparated(rating);
            has_fields = true;
        }
        if let Some(comment) = review.comment {
            separated.push("comment = ").push_bind_unseparated(comment);
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
