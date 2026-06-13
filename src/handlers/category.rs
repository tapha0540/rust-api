use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use crate::{
    models::category::Category,
    repository,
    services::logger::Logger,
    types::{ApiResponse, AppState},
};

pub async fn create(Json(category): Json<Category>) -> (StatusCode, Json<ApiResponse<Category>>) {
    let resp = ApiResponse {
        message: "Category created".to_string(),
        data: Some(category),
    };

    (StatusCode::CREATED, Json(resp))
}

pub async fn get_categories(
    State(state): State<AppState>,
) -> (StatusCode, Json<ApiResponse<Vec<Category>>>) {
    match repository::category::get_categories(&state.db).await {
        Ok(categories) => (
            StatusCode::OK,
            Json(ApiResponse {
                message: "Categories fetched".to_string(),
                data: Some(categories),
            }),
        ),
        Err(err) => {
            let mut logger = Logger::new("error.log");
            logger.log(format!("{:?}", err));
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse {
                    message: "product Categories fetching failed".to_string(),
                    data: None,
                }),
            )
        }
    }
}

pub async fn get_category(Path(id): Path<u32>) -> (StatusCode, Json<ApiResponse<()>>) {
    let resp = ApiResponse {
        message: format!("category id {}", id),
        data: None,
    };

    (StatusCode::OK, Json(resp))
}

pub async fn update() -> (StatusCode, Json<ApiResponse<()>>) {
    let resp = ApiResponse::<()> {
        message: "updated".to_string(),
        data: None,
    };

    (StatusCode::OK, Json(resp))
}

pub async fn delete() -> (StatusCode, Json<ApiResponse<()>>) {
    let resp = ApiResponse::<()> {
        message: "deleted".to_string(),
        data: None,
    };

    (StatusCode::OK, Json(resp))
}
