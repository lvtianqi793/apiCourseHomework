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
    pub like_count: i64,
    pub liked: bool,
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
    pub like_count: i64,
    pub liked_count: i64,
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
            like_count: self.like_count,
            liked: self.liked_count > 0,
            created_at: self.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
        }
    }
}

#[derive(Debug, sqlx::FromRow)]
pub struct VideoOwnerRow {
    pub user_id: i64,
    pub video_url: String,
}
