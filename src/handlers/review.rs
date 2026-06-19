use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use tracing::{error, info, warn};

use crate::{
    handlers::Handler,
    models::review::Review,
    repository::review::ReviewRepository,
    types::{ApiResponse, AppState},
};

pub struct ReviewHandler;

impl Handler<Review> for ReviewHandler {
    async fn create(
        State(state): State<AppState>,
        Json(payload): Json<Review>,
    ) -> (StatusCode, Json<ApiResponse<u32>>) {
        let (Some(product_id), Some(user_id), Some(rating), Some(comment)) = (
            payload.product_id,
            payload.user_id,
            payload.rating,
            payload.comment,
        ) else {
            warn!(
                "Verify your request body to create a new Review because some variables are missing."
            );
            return (
                StatusCode::NOT_ACCEPTABLE,
                Json(ApiResponse::new(
                    "Error some variables are missing from your request body.",
                    None,
                )),
            );
        };

        match ReviewRepository::insert(&state.db, product_id, user_id, rating, comment).await {
            Ok(res) => {
                info!("Request to create a new Review was successful.");
                (
                    StatusCode::CREATED,
                    Json(ApiResponse::new(
                        "Review created",
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
    ) -> (StatusCode, Json<ApiResponse<Vec<Review>>>) {
        match ReviewRepository::find_all(&state.db).await {
            Ok(val) => {
                info!("Reviews found.");
                (
                    StatusCode::OK,
                    Json(ApiResponse::new("Reviews fetched", Some(val))),
                )
            }
            Err(err) => {
                error!("{:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponse::new(
                        "Error server: Failed to fetch Reviews.",
                        None,
                    )),
                )
            }
        }
    }
    async fn get_one(
        State(state): State<AppState>,
        Path(id): Path<i32>,
    ) -> (StatusCode, Json<ApiResponse<Review>>) {
        match ReviewRepository::find_by_id(&state.db, id).await {
            Ok(review) => {
                info!("Review found.");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponse::new("Review found", Some(review))),
                )
            }
            Err(err) => {
                warn!("{:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponse::new("Error: Review not found", None)),
                )
            }
        }
    }
    async fn update(
        State(state): State<AppState>,
        Path(id): Path<i32>,
        Json(payload): Json<Review>,
    ) -> (StatusCode, Json<ApiResponse<u32>>) {
        match ReviewRepository::update(&state.db, payload, id).await {
            Ok(res) => {
                if res.rows_affected() == 0u64 {
                    info!("id to update Review is does not exist.");
                    (
                        StatusCode::NOT_FOUND,
                        Json(ApiResponse::new(
                            "The id you provided does not exist in Reviews table",
                            None,
                        )),
                    )
                } else {
                    info!("Review updated.");
                    (
                        StatusCode::OK,
                        Json(ApiResponse::new(
                            format!("Review with id {} has been updated.", id).as_str(),
                            Some(id as u32),
                        )),
                    )
                }
            }
            Err(err) => {
                error!("{:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponse::new("Server error: Review not updated.", None)),
                )
            }
        }
    }

    async fn delete(
        State(state): State<AppState>,
        Path(id): Path<i32>,
    ) -> (StatusCode, Json<ApiResponse<u32>>) {
        match ReviewRepository::delete(&state.db, id).await {
            Ok(res) => {
                if res.rows_affected() == 0u64 {
                    warn!("the id provided to delete a Review did not exist in the table");
                    (
                        StatusCode::NOT_FOUND,
                        Json(ApiResponse::new(
                            "The id you provided does not exist in Reviews table",
                            None,
                        )),
                    )
                } else {
                    info!("A Review was deleted.");
                    let id = res.last_insert_id();
                    (
                        StatusCode::OK,
                        Json(ApiResponse::new(
                            format!("Review with id {} has been deleted.", id).as_str(),
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
