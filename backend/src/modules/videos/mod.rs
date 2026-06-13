pub mod handlers;
pub mod models;

use axum::{
    routing::{delete, get, patch, post},
    Router,
};

use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/videos/recommend/next", get(handlers::recommend_next))
        .route("/videos/recommend/prev", get(handlers::recommend_prev))
        .route("/videos/by-uuid/:uuid", get(handlers::get_video_by_uuid))
        .route(
            "/videos/:id/like",
            post(handlers::like_video).delete(handlers::unlike_video),
        )
        .route(
            "/videos/:id/comments",
            get(handlers::list_comments).post(handlers::post_comment),
        )
        .route(
            "/videos/:id/favorite",
            post(handlers::favorite_video).delete(handlers::unfavorite_video),
        )
        .route(
            "/videos/:id/danmaku",
            get(handlers::list_danmaku).post(handlers::send_danmaku),
        )
        .route("/users/:id", get(handlers::get_user_profile))
        .route("/my/videos", post(handlers::publish_video).get(handlers::list_my_videos))
        .route(
            "/my/videos/:id",
            patch(handlers::update_my_video).delete(handlers::delete_my_video),
        )
        .route("/my/favorites", get(handlers::list_my_favorites))
        .route("/videos/:id/share", post(handlers::increment_share))
        .route("/search", get(handlers::search))
}
