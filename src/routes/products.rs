use crate::{handlers::product::ProductHandler, types::AppState};
use axum::{
    Router,
    routing::{delete, get, post, put},
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", post(ProductHandler::create))
        .route("/", get(ProductHandler::get_products))
        .route("/{id}", get(ProductHandler::get_product))
        .route("/{id}", put(ProductHandler::update))
        .route("/{id}", delete(ProductHandler::delete))
}
