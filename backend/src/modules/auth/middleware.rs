use axum::{
    body::Body,
    extract::State,
    http::{header::AUTHORIZATION, Request},
    middleware::Next,
    response::Response,
};

use crate::{
    error::AppError,
    modules::auth::models::AuthUser,
    utils::jwt::decode_token,
    AppState,
};

pub async fn require_auth(
    State(state): State<AppState>,
    mut request: Request<Body>,
    next: Next,
) -> Result<Response, AppError> {
    let token = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
        .ok_or(AppError::Unauthorized)?;

    let claims = decode_token(token, &state.config.jwt_secret)?;

    request.extensions_mut().insert(AuthUser {
        id: claims.sub,
    });

    Ok(next.run(request).await)
}
