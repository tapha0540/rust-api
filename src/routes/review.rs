use crate::{
    handlers::{Handler, review::ReviewHandler},
    middlewares::user::UserMiddlewares,
    types::AppState,
};
use axum::{
    Router, middleware,
    routing::{delete, get, post, put},
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/{id}", delete(ReviewHandler::delete))
        .route("/", post(ReviewHandler::create))
        .route("/", get(ReviewHandler::get_all))
        .route("/{id}", get(ReviewHandler::get_one))
        .route("/{id}", put(ReviewHandler::update))
}
