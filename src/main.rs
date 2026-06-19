mod database;
mod handlers;
mod middlewares;
mod models;
mod repository;
mod routes;
mod tests;
mod types;
mod utils;

use axum::Router;
use std::env;
use tracing::info;

use crate::{
    database::connect_db,
    types::AppState,
    utils::{cors::get_cors, logger::setting_up_logger},
};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    dotenvy::dotenv().ok();

    let _log_guard = setting_up_logger();

    let state = AppState {
        db: connect_db().await,
    };

    let app = Router::new()
        .nest("/auth", routes::auth::routes())
        .nest("/categories", routes::category::routes())
        .nest("/products", routes::products::routes())
        .nest("/reviews", routes::review::routes())
        .nest("/payments", routes::products::routes())
        .with_state(state)
        .layer(get_cors());

    let listener = tokio::net::TcpListener::bind(
        env::var("ADDR").expect("The environment variable addr is not set"),
    )
    .await
    .unwrap();

    axum::serve(listener, app).await.unwrap();
}
