use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use tracing::{error, info, warn};

use crate::{
    handlers::Handler,
    models::product::Product,
    repository::product::ProductRepository,
    types::{ApiResponse, AppState},
};

pub struct ProductHandler;

impl Handler<Product> for ProductHandler {
    async fn create(
        State(state): State<AppState>,
        Json(payload): Json<Product>,
    ) -> (StatusCode, Json<ApiResponse<u32>>) {
        let (
            Some(name),
            Some(description),
            Some(price),
            Some(stock),
            Some(category_id),
            Some(image_url),
        ) = (
            payload.name,
            payload.description,
            payload.price,
            payload.stock,
            payload.category_id,
            payload.image_url,
        )
        else {
            warn!(
                "Verify your request body to create a new product because some variables are missing."
            );
            return (
                StatusCode::NOT_ACCEPTABLE,
                Json(ApiResponse::new(
                    "Error some variables are missing from your request body.",
                    None,
                )),
            );
        };

        match ProductRepository::insert(
            &state.db,
            name,
            description,
            price,
            stock,
            category_id,
            image_url,
        )
        .await
        {
            Ok(res) => {
                info!("Request to create a new Product was successful.");
                (
                    StatusCode::CREATED,
                    Json(ApiResponse::new(
                        "Product created",
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
    ) -> (StatusCode, Json<ApiResponse<Vec<Product>>>) {
        match ProductRepository::find_products(&state.db).await {
            Ok(val) => {
                info!("Products found.");
                (
                    StatusCode::OK,
                    Json(ApiResponse::new("Products fetched", Some(val))),
                )
            }
            Err(err) => {
                error!("{:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponse::new(
                        "Error server: Failed to fetch products.",
                        None,
                    )),
                )
            }
        }
    }
    async fn get_one(
        State(state): State<AppState>,
        Path(id): Path<i32>,
    ) -> (StatusCode, Json<ApiResponse<Product>>) {
        match ProductRepository::find_product_by_id(&state.db, id).await {
            Ok(product) => {
                info!("Product found.");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponse::new("Product found", Some(product))),
                )
            }
            Err(err) => {
                warn!("{:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponse::new("Error: product not found", None)),
                )
            }
        }
    }
    async fn update(
        State(state): State<AppState>,
        Path(id): Path<i32>,
        Json(payload): Json<Product>,
    ) -> (StatusCode, Json<ApiResponse<u32>>) {
        match ProductRepository::update(&state.db, payload, id).await {
            Ok(res) => {
                if res.rows_affected() == 0u64 {
                    info!("id to update Product is does not exist.");
                    (
                        StatusCode::NOT_FOUND,
                        Json(ApiResponse::new(
                            "The id you provided does not exist in products table",
                            None,
                        )),
                    )
                } else {
                    info!("Product updated.");
                    (
                        StatusCode::OK,
                        Json(ApiResponse::new(
                            format!("Product with id {} has been updated.", id).as_str(),
                            Some(id as u32),
                        )),
                    )
                }
            }
            Err(err) => {
                error!("{:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponse::new("Server error: product not updated.", None)),
                )
            }
        }
    }

    async fn delete(
        State(state): State<AppState>,
        Path(id): Path<i32>,
    ) -> (StatusCode, Json<ApiResponse<u32>>) {
        match ProductRepository::delete(&state.db, id).await {
            Ok(res) => {
                if res.rows_affected() == 0u64 {
                    warn!("the id provided to delete a Product did not exist in the table");
                    (
                        StatusCode::NOT_FOUND,
                        Json(ApiResponse::new(
                            "The id you provided does not exist in products table",
                            None,
                        )),
                    )
                } else {
                    info!("A Product was deleted.");
                    let id = res.last_insert_id();
                    (
                        StatusCode::OK,
                        Json(ApiResponse::new(
                            format!("Product with id {} has been deleted.", id).as_str(),
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
