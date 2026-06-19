use axum::{Router, routing::post};

use crate::{AppState, handlers::auth::AuthHandler};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/login", post(AuthHandler::log_in))
        .route("/signin", post(AuthHandler::sign_in))
        .route("/logout", post(AuthHandler::log_out))
}
