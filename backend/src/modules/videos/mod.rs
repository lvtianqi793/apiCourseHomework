pub mod handlers;
pub mod models;

use axum::{
    routing::{delete, get, post},
    Router,
};

use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/videos/recommend/next", get(handlers::recommend_next))
        .route("/videos/recommend/prev", get(handlers::recommend_prev))
        .route(
            "/videos/:id/like",
            post(handlers::like_video).delete(handlers::unlike_video),
        )
        .route("/my/videos", post(handlers::publish_video).get(handlers::list_my_videos))
        .route("/my/videos/:id", delete(handlers::delete_my_video))
}
