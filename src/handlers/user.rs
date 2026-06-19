use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use tracing::{error, info, warn};

use crate::{
    handlers::Handler,
    models::user::{User, UserRole},
    repository::user::UserRepository,
    types::{ApiResponse, AppState},
};

pub struct UserHandler;

impl Handler<User> for UserHandler {
    async fn create(
        State(state): State<AppState>,
        Json(payload): Json<User>,
    ) -> (StatusCode, Json<ApiResponse<u32>>) {
        let (
            Some(first_name),
            Some(last_name),
            Some(email),
            Some(phone),
            Some(password),
            Some(role_str),
        ) = (
            payload.first_name,
            payload.last_name,
            payload.email,
            payload.phone,
            payload.password,
            payload.role,
        )
        else {
            warn!(
                "Verify your request body to create a new User because some variables are missing."
            );
            return (
                StatusCode::NOT_ACCEPTABLE,
                Json(ApiResponse::new(
                    "Error some variables are missing from your request body.",
                    None,
                )),
            );
        };

        let role = UserRole::new(role_str).unwrap_or_else(|| {
            error!("Trying to create a user with invalid role.");
            UserRole::Customer
        });

        match UserRepository::insert(
            &state.db, first_name, last_name, email, password, &role, phone,
        )
        .await
        {
            Ok(res) => {
                info!("Request to create a new User was successful.");
                (
                    StatusCode::OK,
                    Json(ApiResponse::new(
                        "User created",
                        Some(res.last_insert_id() as u32),
                    )),
                )
            }
            Err(err) => {
                error!("{err:?}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponse::new("Server Error", None)),
                )
            }
        }
    }

    async fn get_all(State(state): State<AppState>) -> (StatusCode, Json<ApiResponse<Vec<User>>>) {
        match UserRepository::find_all(&state.db).await {
            Ok(val) => {
                info!("Users found.");
                (
                    StatusCode::OK,
                    Json(ApiResponse::new("Users fetched", Some(val))),
                )
            }
            Err(err) => {
                error!("{:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponse::new(
                        "Error server: Failed to fetch Users.",
                        None,
                    )),
                )
            }
        }
    }

    async fn get_one(
        State(state): State<AppState>,
        Path(id): Path<i32>,
    ) -> (StatusCode, Json<ApiResponse<User>>) {
        match UserRepository::find(&state.db, Some(id), None, None).await {
            Ok(mut user) => {
                info!("User found.");
                // we get rid of the password for security reasons.
                user.password = None;
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponse::new("User found", Some(user))),
                )
            }
            Err(err) => {
                warn!("{:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponse::new("Error: User not found", None)),
                )
            }
        }
    }

    async fn update(
        State(state): State<AppState>,
        Path(id): Path<i32>,
        Json(payload): Json<User>,
    ) -> (StatusCode, Json<ApiResponse<u32>>) {
        match UserRepository::update(&state.db, payload, id).await {
            Ok(res) => {
                if res.rows_affected() == 0u64 {
                    info!("id to update User is does not exist.");
                    (
                        StatusCode::NOT_FOUND,
                        Json(ApiResponse::new(
                            "The id you provided does not exist in Users table",
                            None,
                        )),
                    )
                } else {
                    let id = res.last_insert_id() as u32;
                    info!("User updated.");
                    (
                        StatusCode::OK,
                        Json(ApiResponse::new(
                            format!("User with id {} has been updated.", id).as_str(),
                            Some(id),
                        )),
                    )
                }
            }
            Err(err) => {
                error!("{:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponse::new("Server error: User not updated.", None)),
                )
            }
        }
    }

    async fn delete(
        State(state): State<AppState>,
        Path(id): Path<i32>,
    ) -> (StatusCode, Json<ApiResponse<u32>>) {
        match UserRepository::delete(&state.db, id).await {
            Ok(res) => {
                if res.rows_affected() == 0u64 {
                    warn!("the id provided to delete a User did not exist in the table");
                    (
                        StatusCode::NOT_FOUND,
                        Json(ApiResponse::new(
                            "The id you provided does not exist in Users table",
                            None,
                        )),
                    )
                } else {
                    info!("A User was deleted.");
                    let id = res.last_insert_id();
                    (
                        StatusCode::OK,
                        Json(ApiResponse::new(
                            format!("User with id {} has been deleted.", id).as_str(),
                            Some(id as u32),
                        )),
                    )
                }
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
}
