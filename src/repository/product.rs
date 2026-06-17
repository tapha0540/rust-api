use rust_decimal::Decimal;
use sqlx::{MySql, Pool, QueryBuilder, mysql::MySqlQueryResult, query, query_as};

use crate::models::product::Product;

pub struct ProductRepository;

impl ProductRepository {
    pub async fn insert(
        pool: &Pool<MySql>,
        name: String,
        description: String,
        price: Decimal,
        stock: i32,
        category_id: i16,
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

    pub async fn find_product_by_id(pool: &Pool<MySql>, id: i32) -> Result<Product, sqlx::Error> {
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

    pub async fn delete(pool: &Pool<MySql>, id: i32) -> Result<MySqlQueryResult, sqlx::Error> {
        query("DELETE FROM products WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await
    }

    pub async fn update(
    pool: &Pool<MySql>,
    product: Product,
    id: i32,
) -> Result<MySqlQueryResult, sqlx::Error> {
    // Notez l'espace après SET pour éviter les collages de chaînes
    let mut query_builder = QueryBuilder::<MySql>::new("UPDATE products SET ");

    let mut separated = query_builder.separated(", ");
    let mut has_fields = false;

    if let Some(name) = product.name {
        separated.push("name");
        separated.push_unseparated(" = ");
        separated.push_bind_unseparated(name);
        has_fields = true;
    }

    if let Some(description) = product.description {
        separated.push("description");
        separated.push_unseparated(" = ");
        separated.push_bind_unseparated(description);
        has_fields = true;
    }

    if let Some(price) = product.price {
        separated.push("price");
        separated.push_unseparated(" = ");
        separated.push_bind_unseparated(price);
        has_fields = true;
    }

    if let Some(stock) = product.stock {
        separated.push("stock");
        separated.push_unseparated(" = ");
        separated.push_bind_unseparated(stock);
        has_fields = true;
    }

    if let Some(category_id) = product.category_id {
        separated.push("category_id");
        separated.push_unseparated(" = ");
        separated.push_bind_unseparated(category_id);
        has_fields = true;
    }

    if let Some(image_url) = product.image_url {
        separated.push("image_url");
        separated.push_unseparated(" = ");
        separated.push_bind_unseparated(image_url);
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
