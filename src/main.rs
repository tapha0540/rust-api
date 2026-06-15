mod database;
mod handlers;
mod middlewares;
mod models;
mod repository;
mod routes;
mod services;
mod tests;
mod types;

use axum::Router;
use std::env;

use crate::{
    database::connect_db,
    services::logger::Logger,
    tests::category::delete_category_test,
    types::AppState,
};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    dotenvy::dotenv().ok();

    let state = AppState {
        db: connect_db().await,
        logger: Logger::new(
            env::var("LOG_FILE_PATH").expect("environment variable LOG_FILE_PATH is not set"),
        ),
    };

    let app = Router::new()
        .nest("/auth", routes::auth::routes())
        .nest("/categories", routes::category::routes())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(
        env::var("ADDR").expect("The environment variable addr is not set"),
    )
    .await
    .unwrap();

    axum::serve(listener, app).await.unwrap();
}
