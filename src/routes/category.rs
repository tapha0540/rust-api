use axum::{
    Router,
    routing::{delete, get, post, put},
};

use crate::{
    AppState,
    handlers::{Handler, category::CategoryHandler},
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", post(CategoryHandler::create))
        .route("/", get(CategoryHandler::get_all))
        .route("/{id}", get(CategoryHandler::get_one))
        .route("/{id}", put(CategoryHandler::update))
        .route("/{id}", delete(CategoryHandler::delete))
}
