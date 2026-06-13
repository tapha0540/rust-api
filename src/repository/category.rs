use sqlx::{MySql, Pool};

use crate::models::category::Category;

pub async fn get_categories(pool: &Pool<MySql>) -> Result<Vec<Category>, sqlx::Error> {
    let categories = sqlx::query_as::<_, Category>("SELECT * FROM categories")
        .fetch_all(pool)
        .await?;

    Ok(categories)
}
