use std::env;

use chrono::Utc;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};
use tracing::error;

use crate::models::user::UserRole;

const ONE_MONTH: i64 = 60 * 60 * 24 * 30;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,
    pub role: String,
    pub exp: usize,
}

fn get_encoding_key() -> EncodingKey {
    EncodingKey::from_secret(
        env::var("SECRET_KEY")
            .expect("Environment SECRET_KEY is not set.")
            .as_bytes(),
    )
}
fn get_decoding_key() -> DecodingKey {
    DecodingKey::from_secret(
        env::var("SECRET_KEY")
            .expect("Environment SECRET_KEY is not set.")
            .as_bytes(),
    )
}

pub fn get_token(user_id: i32, user_role: UserRole) -> Option<String> {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(ONE_MONTH))
        .expect("Invalid timestamp calculation")
        .timestamp() as usize;

    let my_claims = Claims {
        sub: user_id,
        role: user_role.to_string(),
        exp: expiration,
    };

    match encode(&Header::default(), &my_claims, &get_encoding_key()) {
        Ok(val) => Some(val),
        Err(err) => {
            error!("{:?}", err);
            None
        }
    }
}

pub fn decode_token(token: &str) -> Option<Claims> {
    match jsonwebtoken::decode::<Claims>(
        token,
        &get_decoding_key(),
        &jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256),
    ) {
        Ok(val) => Some(val.claims),
        Err(err) => {
            tracing::error!("{:?}", err);
            None
        }
    }
}
