use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};

use crate::types::{ApiResponse, AppState};

pub mod auth;
pub mod category;
pub mod order;
pub mod payment;
pub mod product;
pub mod review;
pub mod user;

pub(crate) trait Handler<T>
where
    T: Serialize + for<'a> Deserialize<'a> + Sync + Send + 'static,
{
    async fn create(
        state: State<AppState>,
        payload: Json<T>,
    ) -> (StatusCode, Json<ApiResponse<u32>>);

    async fn get_all(state: State<AppState>) -> (StatusCode, Json<ApiResponse<Vec<T>>>);

    async fn get_one(
        state: State<AppState>,
        id: Path<i32>,
    ) -> (StatusCode, Json<ApiResponse<T>>);

    async fn update(
        state: State<AppState>,
        id: Path<i32>,
        payload: Json<T>,
    ) -> (StatusCode, Json<ApiResponse<u32>>);

    async fn delete(state: State<AppState>, id: Path<i32>) -> (StatusCode, Json<ApiResponse<u32>>);
}
