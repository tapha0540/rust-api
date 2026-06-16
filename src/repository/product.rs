use sqlx::{MySql, Pool, QueryBuilder, mysql::MySqlQueryResult, query, query_as};

use crate::models::product::Product;

pub struct ProductRepository;

impl ProductRepository {
    pub async fn insert(
        pool: &Pool<MySql>,
        name: String,
        description: String,
        price: f32,
        stock: u32,
        category_id: u16,
        image_url: String,
    ) -> Result<MySqlQueryResult, sqlx::Error> {
        query("INSERT INTO products(name, description, price, stock, category_id, image_url) VALUES (?,?,?,?,?,?)")
    .bind(name)
    .bind(description)
    .bind(price)
    .bind(stock)
    .bind(category_id)
    .bind(image_url)
    .execute(pool).await
    }

    pub async fn find_product_by_id(pool: &Pool<MySql>, id: u32) -> Result<Product, sqlx::Error> {
        query_as::<'_, MySql, Product>("SELECT id, name, description, price, stock, category_id, image_url, created_at, updated_at FROM products WHERE id = ?")
    .bind(id)
    .fetch_one(pool)
    .await
    }

    pub async fn find_products(pool: &Pool<MySql>) -> Result<Vec<Product>, sqlx::Error> {
        query_as::<'_, MySql, Product>("SELECT id, name, description, price, stock, category_id, image_url, created_at, updated_at FROM products")
    .fetch_all(pool)
    .await
    }

    pub async fn delete(pool: &Pool<MySql>, id: u32) -> Result<MySqlQueryResult, sqlx::Error> {
        query("DELETE FROM products WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await
    }

    pub async fn updated(
        pool: &Pool<MySql>,
        product: Product,
        id: u32,
    ) -> Result<MySqlQueryResult, sqlx::Error> {
        let mut query_builder = QueryBuilder::<MySql>::new("UPDATE product SET");

        let mut separated = query_builder.separated(", ");
        let mut has_fields = false;

        if let Some(name) = product.name {
            separated.push("name = ").push_bind(name);
            has_fields = true;
        }

        if let Some(description) = product.description {
            separated.push("description = ").push_bind(description);
            has_fields = true;
        }
        if let Some(price) = product.price {
            separated.push("price = ").push_bind(price);
            has_fields = true;
        }
        if let Some(stock) = product.stock {
            separated.push("stock = ").push_bind(stock);
            has_fields = true;
        }
        if let Some(category_id) = product.category_id {
            separated.push("category_id = ").push_bind(category_id);
            has_fields = true;
        }
        if let Some(image_url) = product.image_url {
            separated.push("image_url = ").push_bind(image_url);
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
