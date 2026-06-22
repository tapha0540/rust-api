use axum::{
    Router, middleware,
    routing::{delete, get, post, put},
};

use crate::{AppState, handlers::order::OrderHandler, middlewares::user::UserMiddlewares};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/{id}", delete(OrderHandler::delete))
        .layer(middleware::from_fn(UserMiddlewares::admin_only))
        // Above admin accessed is required
        .route("/", post(OrderHandler::create))
        .route("/", get(OrderHandler::get_all))
        .route("/{id}", get(OrderHandler::get_one))
        .route("/{id}", put(OrderHandler::update))  
        .layer(middleware::from_fn(UserMiddlewares::get_user_from_token))
}
