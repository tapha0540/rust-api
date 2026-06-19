use crate::{handlers::Handler, models::order::Order};


pub struct OrderHandler;

impl Handler<Order> for OrderHandler {
    async fn create(
        state: axum::extract::State<crate::types::AppState>,
        payload: axum::Json<Order>,
    ) -> (axum::http::StatusCode, axum::Json<crate::types::ApiResponse<u32>>) {
        todo!()
    }

    async fn get_all(state: axum::extract::State<crate::types::AppState>) -> (axum::http::StatusCode, axum::Json<crate::types::ApiResponse<Vec<Order>>>) {
        todo!()
    }

    async fn get_one(
        state: axum::extract::State<crate::types::AppState>,
        id: axum::extract::Path<i32>,
    ) -> (axum::http::StatusCode, axum::Json<crate::types::ApiResponse<Order>>) {
        todo!()
    }

    async fn update(
        state: axum::extract::State<crate::types::AppState>,
        id: axum::extract::Path<i32>,
        payload: axum::Json<Order>,
    ) -> (axum::http::StatusCode, axum::Json<crate::types::ApiResponse<u32>>) {
        todo!()
    }

    async fn delete(state: axum::extract::State<crate::types::AppState>, id: axum::extract::Path<i32>) -> (axum::http::StatusCode, axum::Json<crate::types::ApiResponse<u32>>) {
        todo!()
    }
}