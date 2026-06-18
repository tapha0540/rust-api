use argon2::{Argon2, PasswordHasher, password_hash};
use axum::{Json, extract::State, http::StatusCode};
use tracing::{error, info};

use crate::{
    models::user::{User, UserRole},
    repository::user::UserRepository,
    types::{ApiResponse, AppState},
    utils::hash::password_hash,
};

pub struct AuthHandler;

impl AuthHandler {
    pub async fn log_in(
        State(state): State<AppState>,
        Json(user): Json<User>,
    ) -> (StatusCode, Json<ApiResponse<u64>>) {
        let (
            Some(first_name),
            Some(last_name),
            Some(email),
            Some(phone),
            Some(role_as_str),
            Some(password),
        ) = (
            user.first_name,
            user.last_name,
            user.email,
            user.phone,
            user.role,
            user.password,
        )
        else {
            return (
                StatusCode::NOT_ACCEPTABLE,
                Json(ApiResponse::new(
                    "Error verify your request body some params are missing.",
                    None,
                )),
            );
        };

        // Verify if a similar account already exists
        if let Ok(similar_user) =
            UserRepository::find(&state.db, None, Some(email.clone()), Some(phone.clone())).await
        {
            let param = if let Some(val) = similar_user.email
                && val == email
            {
                "email"
            } else {
                "phone number"
            };
            info!(
                "A user tried to use {} that already exists to create a new account.",
                param
            );
            return (
                StatusCode::NOT_ACCEPTABLE,
                Json(ApiResponse::new(
                    format!("A user with {} already exists !", param).as_str(),
                    None,
                )),
            );
        }

        let role = UserRole::new(role_as_str).unwrap_or(UserRole::Customer);
        let hash = password_hash(password.as_bytes());

        match UserRepository::insert(&state.db, first_name, last_name, email, hash, role, phone)
            .await
        {
            Ok(query_result) => {
                info!("new User account created");
                (
                    StatusCode::OK,
                    Json(ApiResponse::new(
                        "Account created successfully !",
                        Some(query_result.last_insert_id()),
                    )),
                )
            }
            Err(err) => {
                error!("{:?}", err);
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponse::new("Server error", None)),
                );
            }
        }
    }
    pub async fn sign_in() -> String {
        "Sign in".to_string()
    }
    pub async fn log_out() -> String {
        "Log out".to_string()
    }
}
