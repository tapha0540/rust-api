use axum::{
    Router,
    routing::{delete, get, post, put},
};

use crate::{AppState, handlers::category};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", post(category::create))
        .route("/", get(category::get_categories))
        .route("/{id}", get(category::get_category))
        .route("/{id}", put(category::update))
        .route("/{id}", delete(category::delete))
}
