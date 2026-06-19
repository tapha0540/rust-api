use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use tracing::{error, info, warn};

use crate::{
    handlers::Handler,
    models::payment::{Payment, PaymentMethod, PaymentStatus},
    repository::payment::PaymentRepository,
    types::{ApiResponse, AppState},
};

pub struct PaymentHandler;

impl Handler<Payment> for PaymentHandler {
    async fn create(
        State(state): State<AppState>,
        Json(payload): Json<Payment>,
    ) -> (StatusCode, Json<ApiResponse<u32>>) {
        let (Some(order_id), Some(amount), Some(method_str), Some(status_str)) = (
            payload.order_id,
            payload.amount,
            payload.method,
            payload.status,
        ) else {
            warn!(
                "Verify your request body to create a new Payment because some variables are missing."
            );
            return (
                StatusCode::NOT_ACCEPTABLE,
                Json(ApiResponse::new(
                    "Error some variables are missing from your request body.",
                    None,
                )),
            );
        };

        let (Some(method), Some(status)) = (
            PaymentMethod::new(method_str),
            PaymentStatus::new(status_str),
        ) else {
            error!("user role value from database is corrupted.");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::new("Server Error", None)),
            );
        };

        match PaymentRepository::insert(&state.db, order_id, amount, method, status).await {
            Ok(res) => {
                info!("Request to create a new Payment was successful.");
                (
                    StatusCode::CREATED,
                    Json(ApiResponse::new(
                        "Payment created",
                        Some(res.last_insert_id() as u32),
                    )),
                )
            }
            Err(err) => {
                error!("{:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponse::new("Server Error", None)),
                )
            }
        }
    }

    async fn get_all(
        State(state): State<AppState>,
    ) -> (StatusCode, Json<ApiResponse<Vec<Payment>>>) {
        match PaymentRepository::find_all(&state.db).await {
            Ok(val) => {
                info!("Payments found.");
                (
                    StatusCode::OK,
                    Json(ApiResponse::new("Payments fetched", Some(val))),
                )
            }
            Err(err) => {
                error!("{:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponse::new(
                        "Error server: Failed to fetch Payments.",
                        None,
                    )),
                )
            }
        }
    }
    async fn get_one(
        State(state): State<AppState>,
        Path(id): Path<i32>,
    ) -> (StatusCode, Json<ApiResponse<Payment>>) {
        match PaymentRepository::find_by_id(&state.db, id).await {
            Ok(payment) => {
                info!("Payment found.");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponse::new("Payment found", Some(payment))),
                )
            }
            Err(err) => {
                warn!("{:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponse::new("Error: Payment not found", None)),
                )
            }
        }
    }
    async fn update(
        State(state): State<AppState>,
        Path(id): Path<i32>,
        Json(payload): Json<Payment>,
    ) -> (StatusCode, Json<ApiResponse<u32>>) {
        match PaymentRepository::update(&state.db, payload, id).await {
            Ok(res) => {
                if res.rows_affected() == 0u64 {
                    info!("id to update Payment is does not exist.");
                    (
                        StatusCode::NOT_FOUND,
                        Json(ApiResponse::new(
                            "The id you provided does not exist in Payments table",
                            None,
                        )),
                    )
                } else {
                    info!("Payment updated.");
                    (
                        StatusCode::OK,
                        Json(ApiResponse::new(
                            "Payment has been updated.",
                            Some(id as u32),
                        )),
                    )
                }
            }
            Err(err) => {
                error!("{:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponse::new("Server error: Payment not updated.", None)),
                )
            }
        }
    }

    async fn delete(
        State(state): State<AppState>,
        Path(id): Path<i32>,
    ) -> (StatusCode, Json<ApiResponse<u32>>) {
        match PaymentRepository::delete(&state.db, id).await {
            Ok(res) => {
                if res.rows_affected() == 0u64 {
                    warn!("the id provided to delete a Payment did not exist in the table");
                    (
                        StatusCode::NOT_FOUND,
                        Json(ApiResponse::new(
                            "The id you provided does not exist in Payments table",
                            None,
                        )),
                    )
                } else {
                    info!("A Payment was deleted.");
                    let id = res.last_insert_id();
                    (
                        StatusCode::OK,
                        Json(ApiResponse::new(
                            format!("Payment with id {} has been deleted.", id).as_str(),
                            Some(id as u32),
                        )),
                    )
                }
            }
            Err(err) => {
                tracing::error!("{:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponse::new("Server Error", None)),
                )
            }
        }
    }
}
