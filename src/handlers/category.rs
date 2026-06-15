use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use crate::{
    models::category::Category,
    repository::category::{
        create_category, delete_category, find_categories, find_category_by_id, update_category,
    },
    types::{ApiResponse, AppState},
};

pub async fn create(
    State(mut state): State<AppState>,
    Json(payload): Json<Category>,
) -> (StatusCode, Json<ApiResponse<u64>>) {
    let (Some(name), Some(description), Some(icon_url)) =
        (payload.name, payload.description, payload.icon_url)
    else {
        return (
            StatusCode::NOT_ACCEPTABLE,
            Json(ApiResponse::new(
                "Error Some variable are missing verify your fetch request!",
                None,
            )),
        );
    };

    match create_category(&state.db, name, description, icon_url, payload.parent_id).await {
        Ok(res) => (
            StatusCode::CREATED,
            Json(ApiResponse::new(
                "Category created.",
                Some(res.last_insert_id()),
            )),
        ),
        Err(err) => {
            state.logger.log(err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::new("Server error", None)),
            )
        }
    }
}

pub async fn get_categories(
    State(mut state): State<AppState>,
) -> (StatusCode, Json<ApiResponse<Vec<Category>>>) {
    match find_categories(&state.db).await {
        Ok(categories) => (
            StatusCode::FOUND,
            Json(ApiResponse::new("Categories fetched", Some(categories))),
        ),
        Err(err) => {
            state.logger.log(err);
            (
                StatusCode::NOT_FOUND,
                Json(ApiResponse::new("product Categories fetching failed", None)),
            )
        }
    }
}

pub async fn get_category(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> (StatusCode, Json<ApiResponse<Category>>) {
    match find_category_by_id(&state.db, id).await {
        Ok(category) => (
            StatusCode::OK,
            Json(ApiResponse::new("Operation succeeded", Some(category))),
        ),
        Err(_) => (
            StatusCode::NOT_FOUND,
            Json(ApiResponse::new(
                "Opeartion Failed: May be category not found",
                None,
            )),
        ),
    }
}

pub async fn update(
    State(mut state): State<AppState>,
    Json(payload): Json<Category>,
) -> (StatusCode, Json<ApiResponse<u64>>) {
    if payload.id.is_none() {
        return (
            StatusCode::NOT_ACCEPTABLE,
            Json(ApiResponse::new(
                "Your request does not provide the id of a specific category.",
                None,
            )),
        );
    }

    match update_category(&state.db, payload).await {
        Ok(res) => {
            if res.rows_affected() == 0u64 {
                (
                    StatusCode::NOT_FOUND,
                    Json(ApiResponse::new(
                        "The id you provided does not exist in categories table",
                        None,
                    )),
                )
            } else {
                let id = res.last_insert_id();
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
            state.logger.log(err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::new("", None)),
            )
        }
    }
}

pub async fn delete(
    State(mut state): State<AppState>,
    Path(id): Path<i32>,
) -> (StatusCode, Json<ApiResponse<u64>>) {
    match delete_category(&state.db, id).await {
        Ok(res) => {
            if res.rows_affected() == 0u64 {
                (
                    StatusCode::NOT_FOUND,
                    Json(ApiResponse::new(
                        "The id you provided does not exist in categories table",
                        None,
                    )),
                )
            } else {
                let id = res.last_insert_id();
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
            state.logger.log(err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::new("Server Error", None)),
            )
        }
    }
}
