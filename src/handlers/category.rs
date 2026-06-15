use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use tracing::warn;

use crate::{
    models::category::Category,
    repository::category::CategoryRepository,
    types::{ApiResponse, AppState},
};

pub async fn create(
    State(state): State<AppState>,
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

    match CategoryRepository::insert(&state.db, name, description, icon_url, payload.parent_id)
        .await
    {
        Ok(res) => (
            StatusCode::CREATED,
            Json(ApiResponse::new(
                "Category created.",
                Some(res.last_insert_id()),
            )),
        ),
        Err(err) => {
            tracing::error!("{:?}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::new("Server error", None)),
            )
        }
    }
}

pub async fn get_categories(
    State(state): State<AppState>,
) -> (StatusCode, Json<ApiResponse<Vec<Category>>>) {
    match CategoryRepository::find_categories(&state.db).await {
        Ok(categories) => (
            StatusCode::FOUND,
            Json(ApiResponse::new("Categories fetched", Some(categories))),
        ),
        Err(err) => {
            tracing::error!("{:?}", err);
            (
                StatusCode::NOT_FOUND,
                Json(ApiResponse::new("product Categories fetching failed", None)),
            )
        }
    }
}

pub async fn get_category(
    State(state): State<AppState>,
    Path(id): Path<u32>,
) -> (StatusCode, Json<ApiResponse<Category>>) {
    match CategoryRepository::find_category_by_id(&state.db, id).await {
        Ok(category) => (
            StatusCode::OK,
            Json(ApiResponse::new("Operation succeeded", Some(category))),
        ),
        Err(err) => {
            tracing::warn!("{:?}", err);
            (
                StatusCode::NOT_FOUND,
                Json(ApiResponse::new(
                    "Opeartion Failed: May be category not found",
                    None,
                )),
            )
        }
    }
}

pub async fn update(
    State(state): State<AppState>,
    Json(payload): Json<Category>,
) -> (StatusCode, Json<ApiResponse<i32>>) {
    let Some(id) = payload.id else {
        tracing::warn!("A request trying to update a Category did not provide an id !");
        return (
            StatusCode::NOT_ACCEPTABLE,
            Json(ApiResponse::new(
                "Your request does not provide the id of a specific category.",
                None,
            )),
        );
    };

    match CategoryRepository::update(&state.db, payload, id).await {
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
                Json(ApiResponse::new("", None)),
            )
        }
    }
}

pub async fn delete(
    State(mut state): State<AppState>,
    Path(id): Path<u32>,
) -> (StatusCode, Json<ApiResponse<u64>>) {
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
            tracing::error!("{:?}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::new("Server Error", None)),
            )
        }
    }
}
