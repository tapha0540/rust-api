use std::env;

use sqlx::{MySql, Pool, mysql::MySqlPoolOptions};

pub async fn connect_db() -> Pool<MySql> {
    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL environment variable is not set!");
    return MySqlPoolOptions::new()
        .connect(&database_url)
        .await
        .unwrap();
}
