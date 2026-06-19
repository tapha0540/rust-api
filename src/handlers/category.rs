use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use tracing::{error, info, warn};

use crate::{
    handlers::Handler, models::category::Category, repository::category::CategoryRepository, types::{ApiResponse, AppState}
};

pub struct CategoryHandler;

impl Handler<Category> for CategoryHandler {
    async fn create(
        State(state): State<AppState>,
        Json(payload): Json<Category>,
    ) -> (StatusCode, Json<ApiResponse<u32>>) {
        let (Some(name), Some(description), Some(icon_url)) =
            (payload.name, payload.description, payload.icon_url)
        else {
            warn!(
                "Verify your request body to create a new category because some variables are missing."
            );
            return (
                StatusCode::NOT_ACCEPTABLE,
                Json(ApiResponse::new(
                    "Error Some variables are missing verify your fetch request!",
                    None,
                )),
            );
        };

        match CategoryRepository::insert(&state.db, name, description, icon_url, payload.parent_id)
            .await
        {
            Ok(res) => {
                info!("Your request to create a new category is successful.");
                (
                    StatusCode::OK,
                    Json(ApiResponse::new(
                        "Category created.",
                        Some(res.last_insert_id() as u32),
                    )),
                )
            }
            Err(err) => {
                error!("{:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponse::new("Server error", None)),
                )
            }
        }
    }

    async fn get_all(
        State(state): State<AppState>,
    ) -> (StatusCode, Json<ApiResponse<Vec<Category>>>) {
        match CategoryRepository::find_categories(&state.db).await {
            Ok(categories) => {
                info!("categories Found");
                (
                    StatusCode::OK,
                    Json(ApiResponse::new("Categories fetched", Some(categories))),
                )
            }
            Err(err) => {
                warn!("{:?}", err);
                (
                    StatusCode::NOT_FOUND,
                    Json(ApiResponse::new("product Categories fetching failed", None)),
                )
            }
        }
    }

    async fn get_one(
        State(state): State<AppState>,
        Path(id): Path<i32>,
    ) -> (StatusCode, Json<ApiResponse<Category>>) {
        match CategoryRepository::find_category_by_id(&state.db, id).await {
            Ok(category) => {
                info!("Category found");
                (
                    StatusCode::OK,
                    Json(ApiResponse::new("Category found", Some(category))),
                )
            }
            Err(err) => {
                warn!("{:?}", err);
                (
                    StatusCode::NOT_FOUND,
                    Json(ApiResponse::new("Error: Category not found", None)),
                )
            }
        }
    }

    async fn update(
        State(state): State<AppState>,
        Path(id): Path<i32>,
        Json(payload): Json<Category>,
    ) -> (StatusCode, Json<ApiResponse<u32>>) {
        match CategoryRepository::update(&state.db, payload, id).await {
            Ok(res) => {
                if res.rows_affected() == 0u64 {
                    info!("The id provided to update a Category did exist in the database.");
                    (
                        StatusCode::NOT_FOUND,
                        Json(ApiResponse::new(
                            "The id you provided does not exist in categories table",
                            None,
                        )),
                    )
                } else {
                    let id = res.last_insert_id();
                    info!("A category was updated from database.");
                    (
                        StatusCode::OK,
                        Json(ApiResponse::new(
                            format!("Category with id {} has been updated.", id).as_str(),
                            Some(id as u32),
                        )),
                    )
                }
            }
            Err(err) => {
                error!("{:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponse::new(
                        "Server error: category not updated.",
                        None,
                    )),
                )
            }
        }
    }

    async fn delete(
        State(state): State<AppState>,
        Path(id): Path<i32>,
    ) -> (StatusCode, Json<ApiResponse<u32>>) {
        match CategoryRepository::delete(&state.db, id).await {
            Ok(res) => {
                if res.rows_affected() == 0u64 {
                    warn!("the id provided to delete a Category did not exist in the table");
                    (
                        StatusCode::NOT_FOUND,
                        Json(ApiResponse::new(
                            "The id you provided does not exist in categories table",
                            None,
                        )),
                    )
                } else {
                    info!("A category was deleted from the table categories.");
                    let id = res.last_insert_id() as u32;
                    (
                        StatusCode::OK,
                        Json(ApiResponse::new(
                            format!("Category with id {} has been deleted.", id).as_str(),
                            Some(id),
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
