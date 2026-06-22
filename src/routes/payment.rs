use crate::{
    handlers::{Handler, payment::PaymentHandler}, middlewares::user::UserMiddlewares, types::AppState,
};
use axum::{
    Router, middleware, routing::{delete, get, post, put},
};

pub fn routes() -> Router<AppState> {
    Router::new()
        // Above admin accessed is required
        .route("/", post(PaymentHandler::create))
        .route("/", get(PaymentHandler::get_all))
        .route("/{id}", put(PaymentHandler::update))
        .route("/{id}", get(PaymentHandler::get_one))
        .route("/{id}", delete(PaymentHandler::delete))
        .layer(middleware::from_fn(UserMiddlewares::get_user_from_token))
}
