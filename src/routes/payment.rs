use crate::{
    handlers::payment::PaymentHandler,
    types::AppState,
};
use axum::{
    Router,
    routing::{delete, get, post, put},
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", post(PaymentHandler::create))
        .route("/", get(PaymentHandler::get_payments))
        .route("/{id}", get(PaymentHandler::get_payment))
        .route("/{id}", put(PaymentHandler::update))
        .route("/{id}", delete(PaymentHandler::delete))
}
