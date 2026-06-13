mod database;
mod handlers;
mod middlewares;
mod models;
mod routes;
mod services;
mod types;
mod repository;

use axum::Router;
use std::env;

use crate::{database::connect_db, types::AppState};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    dotenvy::dotenv().ok();

    let state = AppState {
        db: connect_db().await,
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
