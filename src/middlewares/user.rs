use axum::{
    Extension, Json,
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};

use crate::{
    models::user::{User, UserRole},
    types::ApiResponse,
    utils::token::{Claims, decode_token},
};

pub struct UserMiddlewares;

impl UserMiddlewares {
    pub async fn get_user_from_token(mut req: Request, next: Next) -> Response {
        let auth_header = req
            .headers()
            .get("Authorization")
            .and_then(|h| h.to_str().ok());

        let Some(data) = auth_header else {
            return (
                StatusCode::NOT_ACCEPTABLE,
                Json(ApiResponse::<i32>::new(
                    "Missing or invalid Authorization header, valid format 'Bearer TOKEN'.",
                    None,
                )),
            )
                .into_response();
        };

        // Extraction sécurisée du token Bearer
        let data: Vec<&str> = data.split(' ').collect();

        if data.len() < 2 || data[0] != "Bearer" {
            return (
                StatusCode::NOT_ACCEPTABLE,
                Json(ApiResponse::<i32>::new(
                    "invalid Authorization header: the valid format 'Bearer TOKEN'.",
                    None,
                )),
            )
                .into_response();
        }

        let token = data[1];

        let Some(claims) = decode_token(token) else {
            return (
                StatusCode::NOT_ACCEPTABLE,
                Json(ApiResponse::<User>::new("Token invalid or expired.", None)),
            )
                .into_response();
        };

        // On insère les claims pour les handlers suivants
        req.extensions_mut().insert(claims);

        next.run(req).await
    }

    // Version corrigée : On extrait directement l'extension "Claims" injectée par le premier middleware
    pub async fn admin_only(req: Request, next: Next) -> Response {
        // On récupère les claims qui ont été injectés au préalable
        let Some(claims) = req.extensions().get::<Claims>() else {
            return (
                StatusCode::UNAUTHORIZED,
                Json(ApiResponse::<i32>::new(
                    "Unauthorized: Missing authentication claims.",
                    None,
                )),
            )
                .into_response();
        };
        let role = UserRole::new(claims.role.clone()).unwrap();
        // Remplacer par votre propre vérification de rôle (ex: claims.role == Role::Admin)
        if role != UserRole::Admin {
            return (
                StatusCode::FORBIDDEN,
                Json(ApiResponse::<i32>::new(
                    "Forbidden: Admin access required.",
                    None,
                )),
            )
                .into_response();
        }

        // Si tout est bon, on continue la chaîne vers le handler
        next.run(req).await
    }
}
