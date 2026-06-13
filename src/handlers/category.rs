use axum::{Json, extract::Path, http::StatusCode};

use crate::{models::category::Category, types::ApiResponse};

pub async fn create(Json(category): Json<Category>) -> (StatusCode, Json<ApiResponse<Category>>) {
    

    let resp = ApiResponse {
        message: "Category created".to_string(),
        data: Some(category),
    };

    (StatusCode::CREATED, Json(resp))
}

pub async fn get_categories() -> (StatusCode, Json<ApiResponse<Vec<Category>>>) {


    let ans = vec![Category {
        id: 1,
        name: "".to_string(),
        description: "".to_string(),
        parent_id: 1,
        icon_url: "".to_string(),
        created_at: "".to_string(),
        updated_at: "".to_string(),
    }];

    let resp = ApiResponse {
        message: "Categories fetched".to_string(),
        data: Some(ans),
    };

    (StatusCode::OK, Json(resp))
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
