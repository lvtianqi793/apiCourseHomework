use std::time::Instant;

use axum::{
    body::{to_bytes, Body, Bytes},
    extract::State,
    http::{header::AUTHORIZATION, HeaderMap, Request},
    middleware::Next,
    response::Response,
};
use serde_json::Value;

use crate::{modules::logs, utils::jwt::decode_token, AppState};

const LOG_BODY_LIMIT: usize = 1024 * 1024;

pub async fn request_logger(
    State(state): State<AppState>,
    request: Request<Body>,
    next: Next,
) -> Response {
    let started = Instant::now();
    let method = request.method().to_string();
    let path = request.uri().path().to_string();
    let user_id = extract_user_id(request.headers(), &state.config.jwt_secret);
    let is_multipart = request
        .headers()
        .get("content-type")
        .and_then(|value| value.to_str().ok())
        .map(|value| value.to_lowercase().contains("multipart/form-data"))
        .unwrap_or(false);

    let (parts, body) = request.into_parts();
    let (request_body, request) = if is_multipart {
        (
            Some("<multipart/form-data omitted>".to_string()),
            Request::from_parts(parts, body),
        )
    } else {
        match to_bytes(body, LOG_BODY_LIMIT).await {
            Ok(bytes) => {
                let body_text = bytes_to_sanitized_text(&bytes);
                (Some(body_text), Request::from_parts(parts, Body::from(bytes)))
            }
            Err(error) => (
                Some(format!("<request body read error: {}>", error)),
                Request::from_parts(parts, Body::empty()),
            ),
        }
    };

    let response = next.run(request).await;
    let status_code = response.status().as_u16() as i32;
    let (parts, body) = response.into_parts();

    let (response_body, response) = match to_bytes(body, LOG_BODY_LIMIT).await {
        Ok(bytes) => {
            let body_text = bytes_to_sanitized_text(&bytes);
            (Some(body_text), Response::from_parts(parts, Body::from(bytes)))
        }
        Err(error) => (
            Some(format!("<response body read error: {}>", error)),
            Response::from_parts(parts, Body::from(Bytes::new())),
        ),
    };

    let duration_ms = started.elapsed().as_millis() as i64;
    let pool = state.pool.clone();

    tokio::spawn(async move {
        if let Err(error) = sqlx::query(
            r#"
            INSERT INTO request_logs
              (user_id, method, path, request_body, response_body, status_code, duration_ms)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(user_id)
        .bind(method)
        .bind(path)
        .bind(request_body)
        .bind(response_body)
        .bind(status_code)
        .bind(duration_ms)
        .execute(&pool)
        .await
        {
            tracing::warn!("write request log failed: {}", error);
        }
    });

    response
}

fn extract_user_id(headers: &HeaderMap, jwt_secret: &str) -> Option<i64> {
    let token = headers
        .get(AUTHORIZATION)?
        .to_str()
        .ok()?
        .strip_prefix("Bearer ")?;

    decode_token(token, jwt_secret).ok().map(|claims| claims.sub)
}

fn bytes_to_sanitized_text(bytes: &Bytes) -> String {
    let text = String::from_utf8_lossy(bytes).to_string();
    sanitize_text(&text)
}

fn sanitize_text(text: &str) -> String {
    if text.trim().is_empty() {
        return String::new();
    }

    if let Ok(mut value) = serde_json::from_str::<Value>(text) {
        scrub_json(&mut value);
        return value.to_string();
    }

    text.chars().take(LOG_BODY_LIMIT).collect()
}

fn scrub_json(value: &mut Value) {
    match value {
        Value::Object(map) => {
            for (key, value) in map.iter_mut() {
                let lowered = key.to_lowercase();
                if lowered.contains(logs::PASSWORD_KEY) || lowered.contains(logs::TOKEN_KEY) {
                    *value = Value::String("***".to_string());
                } else {
                    scrub_json(value);
                }
            }
        }
        Value::Array(items) => {
            for item in items {
                scrub_json(item);
            }
        }
        _ => {}
    }
}
