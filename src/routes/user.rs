use crate::{handlers::{Handler, review::ReviewHandler, user::UserHandler}, types::AppState};
use axum::{
    Router,
    routing::{delete, get, post, put},
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", post(UserHandler::create))
        .route("/", get(UserHandler::get_all))
        .route("/{id}", get(UserHandler::get_one))
        .route("/{id}", put(UserHandler::update))
        .route("/{id}", delete(UserHandler::delete))
}
