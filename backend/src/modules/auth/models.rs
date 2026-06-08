use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: UserDto,
}

#[derive(Debug, Clone, Serialize)]
pub struct UserDto {
    pub id: i64,
    pub username: String,
}

#[derive(Debug, Clone)]
pub struct AuthUser {
    pub id: i64,
}

#[derive(Debug, sqlx::FromRow)]
pub struct UserRow {
    pub id: i64,
    pub username: String,
    pub password_hash: String,
}
