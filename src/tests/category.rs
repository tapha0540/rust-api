use axum::{
    Json,
    extract::{Path, State},
};

use crate::{handlers::category, models::category::Category, types::AppState};

pub async fn create_category_test(state: &AppState) {
    let _ = category::create(
        State(state.clone()),
        Json(Category {
            id: None,
            name: Some("Test".to_string()),
            description: Some("Test".to_string()),
            parent_id: None,
            icon_url: Some("Test".to_string()),
            created_at: None,
            updated_at: None,
        }),
    )
    .await;
}

pub async fn update_category_test(state: &AppState) {
    let _ = category::update(
        State(state.clone()),
        Json(Category {
            id: Some(18),
            name: None,
            description: Some("updated 2".to_string()),
            parent_id: None,
            icon_url: Some("updated 2".to_string()),
            created_at: None,
            updated_at: None,
        }),
    )
    .await;
}

pub async fn delete_category_test(state: &AppState) {
    let _ = category::delete(State(state.clone()), Path(18)).await;
}
