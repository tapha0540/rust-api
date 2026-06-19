use crate::{handlers::{Handler, review::ReviewHandler}, types::AppState};
use axum::{
    Router,
    routing::{delete, get, post, put},
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", post(ReviewHandler::create))
        .route("/", get(ReviewHandler::get_all))
        .route("/{id}", get(ReviewHandler::get_one))
        .route("/{id}", put(ReviewHandler::update))
        .route("/{id}", delete(ReviewHandler::delete))
}
