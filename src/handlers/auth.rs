use axum::{Json, extract::State, http::StatusCode};
use tracing::{error, info, warn};

use crate::{
    models::user::{User, UserRole},
    repository::user::UserRepository,
    types::{ApiResponse, AppState},
    utils::{
        hash::{password_hash, password_verify},
        token::get_token,
    },
};

pub struct AuthHandler;

impl AuthHandler {
    pub async fn sign_in(
        State(state): State<AppState>,
        Json(payload): Json<User>,
    ) -> (StatusCode, Json<ApiResponse<String>>) {
        let (Some(first_name), Some(last_name), Some(email), Some(phone), Some(password)) = (
            payload.first_name,
            payload.last_name,
            payload.email,
            payload.phone,
            payload.password,
        ) else {
            warn!("Request to /users/login failed because some params are missing.");
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
            let param: &str = if let Some(val) = similar_user.email
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

        let role = UserRole::Customer;
        let hash = password_hash(password.as_bytes());

        match UserRepository::insert(&state.db, first_name, last_name, email, hash, &role, phone)
            .await
        {
            Ok(query_result) => {
                let user_token = get_token(query_result.last_insert_id() as i32, role).unwrap();
                info!("new User account created");

                (
                    StatusCode::OK,
                    Json(ApiResponse::new(
                        "Account created successfully !",
                        Some(user_token),
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
    pub async fn log_in(
        State(state): State<AppState>,
        Json(payload): Json<User>,
    ) -> (StatusCode, Json<ApiResponse<String>>) {
        let (Some(email), Some(password)) = (payload.email, payload.password) else {
            warn!("Request to /users/signin failed because there are missing params.");
            return (
                StatusCode::NOT_ACCEPTABLE,
                Json(ApiResponse::new(
                    "Failed to sign in because the request body probably miss some parameters",
                    None,
                )),
            );
        };

        // find the user with that email or phone
        let Ok(found_user) =
            UserRepository::find(&state.db, None, Some(email), payload.phone.clone()).await
        else {
            return (
                StatusCode::NOT_FOUND,
                Json(ApiResponse::new(
                    format!(
                        "There is no user with that specific email{}.",
                        if payload.phone.is_some() {
                            " or phone number"
                        } else {
                            ""
                        }
                    )
                    .as_str(),
                    None,
                )),
            );
        };

        let (Some(found_user_id), Some(found_user_password), Some(found_user_role)) =
            (found_user.id, found_user.password, found_user.role)
        else {
            error!(
                "A user's password is from database is none, this violates database rules because password must not be NULL"
            );
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::new("Server Error", None)),
            );
        };

        // Verify if the password is correct.
        if !password_verify(password.as_bytes(), &found_user_password) {
            warn!(
                "An user attempt to connect to an account failed beacause of an incorrect password."
            );
            return (
                StatusCode::UNAUTHORIZED,
                Json(ApiResponse::new("The password is incorrect.", None)),
            );
        }

        let user_token = get_token(
            found_user_id,
            UserRole::new(found_user_role).expect("user's role from database is Invalid."),
        )
        .unwrap();

        (
            StatusCode::OK,
            Json(ApiResponse::new("successfully signed in", Some(user_token))),
        )
    }
    pub async fn log_out() -> String {
        "Log out".to_string()
    }
}
