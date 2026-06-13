use axum::{Router, routing::post};

use crate::{AppState, handlers::auth::{log_in, log_out, sign_in}};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/login", post(log_in))
        .route("/signin", post(sign_in))
        .route("/logout", post(log_out))
}
