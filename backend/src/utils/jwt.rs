use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::error::AppError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i64,
    pub username: String,
    pub exp: usize,
}

pub fn create_token(user_id: i64, username: &str, secret: &str) -> Result<String, AppError> {
    let exp = Utc::now()
        .checked_add_signed(Duration::days(7))
        .ok_or_else(|| AppError::Internal("生成 token 过期时间失败".to_string()))?
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id,
        username: username.to_string(),
        exp,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|error| AppError::Internal(error.to_string()))
}

pub fn decode_token(token: &str, secret: &str) -> Result<Claims, AppError> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map(|token_data| token_data.claims)
    .map_err(|_| AppError::Unauthorized)
}
