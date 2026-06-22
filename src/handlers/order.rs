use axum::{Extension, Json, extract::State, http::StatusCode};

use crate::{
    models::order::Order,
    types::{ApiResponse, AppState},
    utils::token::Claims,
};

pub struct OrderHandler;

impl OrderHandler {
    pub async fn create(
        State(state): State<AppState>,
        Json(payload): Json<Order>,
    ) -> (StatusCode, Json<ApiResponse<u32>>) {
        todo!()
    }

    pub async fn get_all(
        state: axum::extract::State<crate::types::AppState>,
        Extension(claims): Extension<Claims>,
    ) -> (
        axum::http::StatusCode,
        axum::Json<crate::types::ApiResponse<Vec<Order>>>,
    ) {
        tracing::info!("{claims:?}");
        (
            StatusCode::OK,
            Json(ApiResponse::new(format!("{claims:?}").as_str(), None)),
        )
    }

    pub async fn get_one(
        state: axum::extract::State<crate::types::AppState>,
        id: axum::extract::Path<i32>,
    ) -> (
        axum::http::StatusCode,
        axum::Json<crate::types::ApiResponse<Order>>,
    ) {
        todo!()
    }

    pub async fn update(
        state: axum::extract::State<crate::types::AppState>,
        id: axum::extract::Path<i32>,
        payload: axum::Json<Order>,
    ) -> (
        axum::http::StatusCode,
        axum::Json<crate::types::ApiResponse<u32>>,
    ) {
        todo!()
    }

    pub async fn delete(
        state: axum::extract::State<crate::types::AppState>,
        id: axum::extract::Path<i32>,
    ) -> (
        axum::http::StatusCode,
        axum::Json<crate::types::ApiResponse<u32>>,
    ) {
        todo!()
    }
}
