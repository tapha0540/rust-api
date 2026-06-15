use std::{fmt::format, ops::Add};

use chrono::{DateTime, Utc};
use sqlx::{
    AssertSqlSafe, MySql, Pool,
    mysql::{MySqlArguments, MySqlQueryResult},
    query,
};

use crate::{handlers::category, models::category::Category};

pub async fn create_category(
    pool: &Pool<MySql>,
    name: String,
    description: String,
    icon_url: String,
    parent_id: Option<i32>,
) -> Result<MySqlQueryResult, sqlx::Error> {
    sqlx::query("INSERT INTO categories(name, description, parent_id, icon_url) VALUES (?,?,?,?)")
        .bind(name)
        .bind(description)
        .bind(parent_id)
        .bind(icon_url)
        .execute(pool)
        .await
}

pub async fn find_categories(pool: &Pool<MySql>) -> Result<Vec<Category>, sqlx::Error> {
    sqlx::query_as::<MySql, Category>(
        "SELECT id, name, description, parent_id, icon_url, created_at, updated_at FROM categories",
    )
    .fetch_all(pool)
    .await
}
pub async fn find_category_by_id(pool: &Pool<MySql>, id: i32) -> Result<Category, sqlx::Error> {
    sqlx::query_as::<MySql, Category>("SELECT id, name, description, parent_id, icon_url, created_at, updated_at FROM categories WHERE id = ?")
        .bind(id)
        .fetch_one(pool)
        .await
}

pub async fn delete_category(pool: &Pool<MySql>, id: i32) -> Result<MySqlQueryResult, sqlx::Error> {
    sqlx::query("DELETE FROM categories WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await
}

pub async fn update_category(
    pool: &Pool<MySql>,
    category: Category,
) -> Result<MySqlQueryResult, sqlx::Error> {
    let Some(id) = category.id else {
        return Err(sqlx::Error::BeginFailed);
    };
    let mut colums: Vec<&str> = Vec::new();

    if category.name.is_some() {
        colums.push("name = ?");
    }
    if category.description.is_some() {
        colums.push("description = ?");
    }
    if category.icon_url.is_some() {
        colums.push("icon_url = ?");
    }
    if category.parent_id.is_some() {
        colums.push("parent_id = ?");
    }

    let sql = format!("UPDATE categories SET {} WHERE id = ?", colums.join(","));

    let mut query: query::Query<'_, MySql, _> = sqlx::query(AssertSqlSafe(sql));

    if let Some(name) = category.name {
        query = query.bind(name);
    }
    if let Some(description) = category.description {
        query = query.bind(description);
    }
    if let Some(icon_url) = category.icon_url {
        query = query.bind(icon_url);
    }
    if let Some(parent_id) = category.parent_id {
        query = query.bind(parent_id);
    }

    query.bind(id).execute(pool).await
}
