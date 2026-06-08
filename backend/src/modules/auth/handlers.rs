use axum::{extract::State, Json};

use crate::{
    error::AppError,
    modules::auth::models::{LoginRequest, LoginResponse, RegisterRequest, UserDto, UserRow},
    utils::{jwt::create_token, password, response::ApiResponse},
    AppState,
};

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    let username = payload.username.trim();
    let raw_password = payload.password.trim();

    if username.len() < 3 || username.len() > 64 {
        return Err(AppError::BadRequest(
            "用户名长度需要在 3 到 64 个字符之间".to_string(),
        ));
    }

    if raw_password.len() < 6 || raw_password.len() > 64 {
        return Err(AppError::BadRequest(
            "密码长度需要在 6 到 64 个字符之间".to_string(),
        ));
    }

    let exists: Option<(i64,)> = sqlx::query_as("SELECT id FROM users WHERE username = ?")
        .bind(username)
        .fetch_optional(&state.pool)
        .await?;

    if exists.is_some() {
        return Err(AppError::BadRequest("用户名已存在".to_string()));
    }

    let password_hash = password::hash_password(raw_password)?;

    sqlx::query("INSERT INTO users (username, password_hash) VALUES (?, ?)")
        .bind(username)
        .bind(password_hash)
        .execute(&state.pool)
        .await?;

    Ok(Json(ApiResponse::message("注册成功")))
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<ApiResponse<LoginResponse>>, AppError> {
    let username = payload.username.trim();

    let user = sqlx::query_as::<_, UserRow>(
        "SELECT id, username, password_hash FROM users WHERE username = ?",
    )
    .bind(username)
    .fetch_optional(&state.pool)
    .await?
    .ok_or(AppError::Unauthorized)?;

    if !password::verify_password(&payload.password, &user.password_hash)? {
        return Err(AppError::Unauthorized);
    }

    let token = create_token(user.id, &user.username, &state.config.jwt_secret)?;

    Ok(Json(ApiResponse::ok(LoginResponse {
        token,
        user: UserDto {
            id: user.id,
            username: user.username,
        },
    })))
}
