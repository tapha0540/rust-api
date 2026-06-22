use crate::{
    handlers::{Handler, product::ProductHandler},
    middlewares::user::UserMiddlewares,
    types::AppState,
};
use axum::{
    Router, middleware,
    routing::{delete, get, post, put},
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", post(ProductHandler::create))
        .route("/{id}", put(ProductHandler::update))
        .route("/{id}", delete(ProductHandler::delete))
        .layer(middleware::from_fn(UserMiddlewares::admin_only))
        .layer(middleware::from_fn(UserMiddlewares::get_user_from_token))
        // Above admin accessed is required
        .route("/", get(ProductHandler::get_all))
        .route("/{id}", get(ProductHandler::get_one))
}
