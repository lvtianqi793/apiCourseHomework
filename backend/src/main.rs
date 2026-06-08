mod config;
mod db;
mod error;
mod middleware;
mod modules;
mod utils;

use axum::{
    extract::DefaultBodyLimit,
    http::HeaderValue,
    middleware as axum_middleware, Router,
};
use config::Config;
use db::create_pool;
use sqlx::MySqlPool;
use std::net::SocketAddr;
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone)]
pub struct AppState {
    pub pool: MySqlPool,
    pub config: Config,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "video_flow_backend=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::from_env()?;
    tokio::fs::create_dir_all(&config.upload_dir).await?;

    let pool = create_pool(&config.database_url).await?;
    let state = AppState { pool, config };

    let protected_routes = modules::videos::router().route_layer(axum_middleware::from_fn_with_state(
        state.clone(),
        modules::auth::middleware::require_auth,
    ));

    let api_routes = Router::new()
        .nest("/auth", modules::auth::router())
        .merge(protected_routes)
        .layer(axum_middleware::from_fn_with_state(
            state.clone(),
            middleware::logging::request_logger,
        ));

    let frontend_origin = state
        .config
        .frontend_origin
        .parse::<HeaderValue>()
        .unwrap_or_else(|_| HeaderValue::from_static("http://localhost:5173"));

    let cors = CorsLayer::new()
        .allow_origin(frontend_origin)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .nest("/api", api_routes)
        .nest_service("/uploads", ServeDir::new("uploads"))
        .layer(DefaultBodyLimit::max(state.config.max_video_size_bytes as usize))
        .layer(cors)
        .with_state(state.clone());

    let addr: SocketAddr = state.config.server_addr.parse()?;
    tracing::info!("backend listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
