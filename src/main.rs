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
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{
    EnvFilter, Registry,
    fmt::{self},
    layer::SubscriberExt,
    util::SubscriberInitExt,
};

use crate::{database::connect_db, types::AppState, utils::cors::get_cors};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    dotenvy::dotenv().ok();

    let _log_guard = setting_up_logger();

    let state = AppState {
        db: connect_db().await,
    };

    info!("Test");

    let app = Router::new()
        .nest("/auth", routes::auth::routes())
        .nest("/categories", routes::category::routes())
        .nest("/products", routes::products::routes())
        .with_state(state)
        // .layer(get_cors())
        .layer(get_cors());

    let listener = tokio::net::TcpListener::bind(
        env::var("ADDR").expect("The environment variable addr is not set"),
    )
    .await
    .unwrap();

    axum::serve(listener, app).await.unwrap();
}

fn setting_up_logger() -> WorkerGuard {
    let file_appender = tracing_appender::rolling::daily("./logs", "app.log");

    // Le guard DOIT être retourné au main pour éviter d'être détruit immédiatement
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    let terminal_layer = fmt::layer().with_writer(std::io::stdout);

    let file_layer = fmt::layer().json().with_writer(non_blocking);

    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| "api=debug,tower_http=info".into());

    // On initialise le global subscriber ici
    Registry::default()
        .with(env_filter)
        .with(terminal_layer)
        .with(file_layer)
        .init();

    tracing::info!("Le système de log double (Console + Fichier) est prêt.");

    guard
}
