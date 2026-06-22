use axum::{
    Router, middleware,
    routing::{delete, get, post, put},
};

use crate::{
    AppState,
    handlers::{Handler, category::CategoryHandler},
    middlewares::user::UserMiddlewares,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", post(CategoryHandler::create))
        .route("/{id}", delete(CategoryHandler::delete))
        .route("/{id}", put(CategoryHandler::update))
        .layer(middleware::from_fn(UserMiddlewares::admin_only))
        .layer(middleware::from_fn(UserMiddlewares::get_user_from_token))
        // Above require admin access.
        .route("/", get(CategoryHandler::get_all))
        .route("/{id}", get(CategoryHandler::get_one))
}
