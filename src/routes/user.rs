use crate::{
    handlers::{Handler, user::UserHandler},
    middlewares::user::UserMiddlewares,
    types::AppState,
};
use axum::{
    Router, middleware,
    routing::{delete, get, post, put},
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", post(UserHandler::create))
        .route("/", get(UserHandler::get_all))
        .route("/{id}", get(UserHandler::get_one))
        .route("/{id}", put(UserHandler::update))
        .route("/{id}", delete(UserHandler::delete))
        .layer(middleware::from_fn(UserMiddlewares::admin_only))
        .layer(middleware::from_fn(UserMiddlewares::get_user_from_token))
}
