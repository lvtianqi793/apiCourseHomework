use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct RecommendQuery {
    pub current_id: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct PrevQuery {
    pub current_id: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct PageQuery {
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct LikeResponse {
    pub liked: bool,
    pub like_count: i64,
}

#[derive(Debug, Serialize)]
pub struct PageResponse<T> {
    pub items: Vec<T>,
    pub page: u32,
    pub page_size: u32,
    pub total: i64,
}

#[derive(Debug, Serialize)]
pub struct VideoDto {
    pub id: i64,
    pub user_id: i64,
    pub author: String,
    pub title: String,
    pub description: Option<String>,
    pub video_url: String,
    pub view_count: i64,
    pub like_count: i64,
    pub liked: bool,
    pub comment_count: i64,
    pub favorite_count: i64,
    pub favorited: bool,
    pub share_count: i64,
    pub created_at: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct VideoRow {
    pub id: i64,
    pub user_id: i64,
    pub author: String,
    pub title: String,
    pub description: Option<String>,
    pub video_url: String,
    pub view_count: i64,
    pub like_count: i64,
    pub liked_count: i64,
    pub comment_count: i64,
    pub favorite_count: i64,
    pub favorited_count: i64,
    pub share_count: i64,
    pub created_at: NaiveDateTime,
}

impl VideoRow {
    pub fn into_dto(self, public_base_url: &str) -> VideoDto {
        let video_url = if self.video_url.starts_with("http://")
            || self.video_url.starts_with("https://")
        {
            self.video_url
        } else {
            format!("{}{}", public_base_url, self.video_url)
        };

        VideoDto {
            id: self.id,
            user_id: self.user_id,
            author: self.author,
            title: self.title,
            description: self.description,
            video_url,
            view_count: self.view_count,
            like_count: self.like_count,
            liked: self.liked_count > 0,
            comment_count: self.comment_count,
            favorite_count: self.favorite_count,
            favorited: self.favorited_count > 0,
            share_count: self.share_count,
            created_at: (self.created_at + chrono::Duration::hours(8))
                .format("%Y-%m-%d %H:%M:%S")
                .to_string(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct UpdateVideoBody {
    pub title: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct VideoOwnerRow {
    pub user_id: i64,
    pub video_url: String,
}

#[derive(Debug, Deserialize)]
pub struct CommentBody {
    pub content: String,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct CommentDto {
    pub id: i64,
    pub user_id: i64,
    pub author: String,
    pub content: String,
    pub created_at: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct CommentRow {
    pub id: i64,
    pub user_id: i64,
    pub author: String,
    pub content: String,
    pub created_at: NaiveDateTime,
}

impl CommentRow {
    pub fn into_dto(self) -> CommentDto {
        CommentDto {
            id: self.id,
            user_id: self.user_id,
            author: self.author,
            content: self.content,
            created_at: (self.created_at + chrono::Duration::hours(8))
                .format("%Y-%m-%d %H:%M:%S")
                .to_string(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct FavoriteResponse {
    pub favorited: bool,
}

#[derive(Debug, Deserialize)]
pub struct DanmakuBody {
    pub content: String,
    pub timestamp_sec: f32,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct DanmakuDto {
    pub id: i64,
    pub user_id: i64,
    pub author: String,
    pub content: String,
    pub timestamp_sec: f32,
}

#[derive(Debug, sqlx::FromRow)]
pub struct DanmakuRow {
    pub id: i64,
    pub user_id: i64,
    pub author: String,
    pub content: String,
    pub timestamp_sec: f32,
}

impl DanmakuRow {
    pub fn into_dto(self) -> DanmakuDto {
        DanmakuDto {
            id: self.id,
            user_id: self.user_id,
            author: self.author,
            content: self.content,
            timestamp_sec: self.timestamp_sec,
        }
    }
}

#[derive(Debug, sqlx::FromRow)]
pub struct UserProfileRow {
    pub id: i64,
    pub username: String,
}

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub q: String,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct UserResultDto {
    pub id: i64,
    pub username: String,
    pub video_count: i64,
}
