use axum::{
    Router,
    routing::{delete, get, post, put},
};

use crate::{AppState, handlers::category::CategoryHandler};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", post(CategoryHandler::create))
        .route("/", get(CategoryHandler::get_categories))
        .route("/{id}", get(CategoryHandler::get_category))
        .route("/{id}", put(CategoryHandler::update))
        .route("/{id}", delete(CategoryHandler::delete))
}
