use std::path::{Path, PathBuf};

use axum::{
    extract::{Extension, Multipart, Path as AxumPath, Query, State},
    Json,
};
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

use crate::{
    error::AppError,
    modules::{
        auth::models::AuthUser,
        videos::models::{
            LikeResponse, PageQuery, PageResponse, PrevQuery, RecommendQuery, VideoDto,
            VideoOwnerRow, VideoRow,
        },
    },
    utils::response::ApiResponse,
    AppState,
};

pub async fn recommend_next(
    State(state): State<AppState>,
    Extension(user): Extension<AuthUser>,
    Query(query): Query<RecommendQuery>,
) -> Result<Json<ApiResponse<VideoDto>>, AppError> {
    let video = match query.current_id {
        Some(current_id) => {
            let next_video = fetch_next_video_in_cycle(&state, user.id, current_id).await?;
            match next_video {
                Some(video) => Some(video),
                None => fetch_first_video_in_cycle(&state, user.id).await?,
            }
        }
        None => fetch_first_video_in_cycle(&state, user.id).await?,
    };

    let Some(video) = video else {
        return Ok(Json(ApiResponse::message("暂无可推荐视频")));
    };

    record_video_view(&state, user.id, video.id).await?;

    Ok(Json(ApiResponse::ok(
        video.into_dto(&state.config.public_base_url),
    )))
}

pub async fn recommend_prev(
    State(state): State<AppState>,
    Extension(user): Extension<AuthUser>,
    Query(query): Query<PrevQuery>,
) -> Result<Json<ApiResponse<VideoDto>>, AppError> {
    let video = match query.current_id {
        Some(current_id) => {
            let prev_video = fetch_prev_video_in_cycle(&state, user.id, current_id).await?;
            match prev_video {
                Some(video) => Some(video),
                None => fetch_last_video_in_cycle(&state, user.id).await?,
            }
        }
        None => fetch_last_video_in_cycle(&state, user.id).await?,
    };

    let Some(video) = video else {
        return Ok(Json(ApiResponse::message("暂无可推荐视频")));
    };

    record_video_view(&state, user.id, video.id).await?;

    Ok(Json(ApiResponse::ok(
        video.into_dto(&state.config.public_base_url),
    )))
}

async fn fetch_first_video_in_cycle(
    state: &AppState,
    current_user_id: i64,
) -> Result<Option<VideoRow>, AppError> {
    sqlx::query_as::<_, VideoRow>(
        r#"
        SELECT
          v.id,
          v.user_id,
          u.username AS author,
          v.title,
          v.description,
          v.video_url,
          v.like_count,
          (
            SELECT COUNT(*)
            FROM video_likes vl
            WHERE vl.user_id = ? AND vl.video_id = v.id
          ) AS liked_count,
          v.created_at
        FROM videos v
        JOIN users u ON u.id = v.user_id
        WHERE v.status = 1
        ORDER BY v.like_count DESC, v.created_at DESC, v.id DESC
        LIMIT 1
        "#,
    )
    .bind(current_user_id)
    .fetch_optional(&state.pool)
    .await
    .map_err(AppError::from)
}

async fn fetch_last_video_in_cycle(
    state: &AppState,
    current_user_id: i64,
) -> Result<Option<VideoRow>, AppError> {
    sqlx::query_as::<_, VideoRow>(
        r#"
        SELECT
          v.id,
          v.user_id,
          u.username AS author,
          v.title,
          v.description,
          v.video_url,
          v.like_count,
          (
            SELECT COUNT(*)
            FROM video_likes vl
            WHERE vl.user_id = ? AND vl.video_id = v.id
          ) AS liked_count,
          v.created_at
        FROM videos v
        JOIN users u ON u.id = v.user_id
        WHERE v.status = 1
        ORDER BY v.like_count ASC, v.created_at ASC, v.id ASC
        LIMIT 1
        "#,
    )
    .bind(current_user_id)
    .fetch_optional(&state.pool)
    .await
    .map_err(AppError::from)
}

async fn fetch_next_video_in_cycle(
    state: &AppState,
    current_user_id: i64,
    current_video_id: i64,
) -> Result<Option<VideoRow>, AppError> {
    sqlx::query_as::<_, VideoRow>(
        r#"
        SELECT
          v.id,
          v.user_id,
          u.username AS author,
          v.title,
          v.description,
          v.video_url,
          v.like_count,
          (
            SELECT COUNT(*)
            FROM video_likes vl
            WHERE vl.user_id = ? AND vl.video_id = v.id
          ) AS liked_count,
          v.created_at
        FROM videos v
        JOIN users u ON u.id = v.user_id
        JOIN videos c ON c.id = ? AND c.status = 1
        WHERE v.status = 1
          AND (
            v.like_count < c.like_count
            OR (v.like_count = c.like_count AND v.created_at < c.created_at)
            OR (v.like_count = c.like_count AND v.created_at = c.created_at AND v.id < c.id)
          )
        ORDER BY v.like_count DESC, v.created_at DESC, v.id DESC
        LIMIT 1
        "#,
    )
    .bind(current_user_id)
    .bind(current_video_id)
    .fetch_optional(&state.pool)
    .await
    .map_err(AppError::from)
}

async fn fetch_prev_video_in_cycle(
    state: &AppState,
    current_user_id: i64,
    current_video_id: i64,
) -> Result<Option<VideoRow>, AppError> {
    sqlx::query_as::<_, VideoRow>(
        r#"
        SELECT
          v.id,
          v.user_id,
          u.username AS author,
          v.title,
          v.description,
          v.video_url,
          v.like_count,
          (
            SELECT COUNT(*)
            FROM video_likes vl
            WHERE vl.user_id = ? AND vl.video_id = v.id
          ) AS liked_count,
          v.created_at
        FROM videos v
        JOIN users u ON u.id = v.user_id
        JOIN videos c ON c.id = ? AND c.status = 1
        WHERE v.status = 1
          AND (
            v.like_count > c.like_count
            OR (v.like_count = c.like_count AND v.created_at > c.created_at)
            OR (v.like_count = c.like_count AND v.created_at = c.created_at AND v.id > c.id)
          )
        ORDER BY v.like_count ASC, v.created_at ASC, v.id ASC
        LIMIT 1
        "#,
    )
    .bind(current_user_id)
    .bind(current_video_id)
    .fetch_optional(&state.pool)
    .await
    .map_err(AppError::from)
}

async fn record_video_view(
    state: &AppState,
    current_user_id: i64,
    video_id: i64,
) -> Result<(), AppError> {
    sqlx::query("INSERT IGNORE INTO video_views (user_id, video_id) VALUES (?, ?)")
        .bind(current_user_id)
        .bind(video_id)
        .execute(&state.pool)
        .await?;

    Ok(())
}

pub async fn like_video(
    State(state): State<AppState>,
    Extension(user): Extension<AuthUser>,
    AxumPath(video_id): AxumPath<i64>,
) -> Result<Json<ApiResponse<LikeResponse>>, AppError> {
    ensure_video_exists(&state, video_id).await?;

    let mut tx = state.pool.begin().await?;

    let result = sqlx::query("INSERT IGNORE INTO video_likes (user_id, video_id) VALUES (?, ?)")
        .bind(user.id)
        .bind(video_id)
        .execute(&mut *tx)
        .await?;

    if result.rows_affected() > 0 {
        sqlx::query("UPDATE videos SET like_count = like_count + 1 WHERE id = ?")
            .bind(video_id)
            .execute(&mut *tx)
            .await?;
    }

    let like_count = fetch_like_count(&mut tx, video_id).await?;
    tx.commit().await?;

    Ok(Json(ApiResponse::ok(LikeResponse {
        liked: true,
        like_count,
    })))
}

pub async fn unlike_video(
    State(state): State<AppState>,
    Extension(user): Extension<AuthUser>,
    AxumPath(video_id): AxumPath<i64>,
) -> Result<Json<ApiResponse<LikeResponse>>, AppError> {
    ensure_video_exists(&state, video_id).await?;

    let mut tx = state.pool.begin().await?;

    let result = sqlx::query("DELETE FROM video_likes WHERE user_id = ? AND video_id = ?")
        .bind(user.id)
        .bind(video_id)
        .execute(&mut *tx)
        .await?;

    if result.rows_affected() > 0 {
        sqlx::query("UPDATE videos SET like_count = GREATEST(like_count - 1, 0) WHERE id = ?")
            .bind(video_id)
            .execute(&mut *tx)
            .await?;
    }

    let like_count = fetch_like_count(&mut tx, video_id).await?;
    tx.commit().await?;

    Ok(Json(ApiResponse::ok(LikeResponse {
        liked: false,
        like_count,
    })))
}

pub async fn publish_video(
    State(state): State<AppState>,
    Extension(user): Extension<AuthUser>,
    mut multipart: Multipart,
) -> Result<Json<ApiResponse<VideoDto>>, AppError> {
    let mut title: Option<String> = None;
    let mut description: Option<String> = None;
    let mut stored_video_url: Option<String> = None;
    let mut saved_path: Option<PathBuf> = None;

    tokio::fs::create_dir_all(&state.config.upload_dir).await?;

    while let Some(mut field) = multipart.next_field().await? {
        let field_name = field.name().unwrap_or_default().to_string();

        match field_name.as_str() {
            "title" => {
                let value = field.text().await?.trim().to_string();
                if !value.is_empty() {
                    title = Some(value);
                }
            }
            "description" => {
                let value = field.text().await?.trim().to_string();
                if !value.is_empty() {
                    description = Some(value);
                }
            }
            "file" => {
                let original_name = field.file_name().unwrap_or_default().to_lowercase();
                let content_type = field
                    .content_type()
                    .map(|value| value.to_string())
                    .unwrap_or_default();

                if !original_name.ends_with(".mp4") && content_type != "video/mp4" {
                    return Err(AppError::BadRequest("只允许上传 mp4 视频文件".to_string()));
                }

                let stored_name = format!("{}.mp4", Uuid::new_v4());
                let target_path = Path::new(&state.config.upload_dir).join(&stored_name);
                let mut file = tokio::fs::File::create(&target_path).await?;
                let mut size: u64 = 0;

                while let Some(chunk) = field.chunk().await? {
                    size += chunk.len() as u64;
                    if size > state.config.max_video_size_bytes {
                        drop(file);
                        let _ = tokio::fs::remove_file(&target_path).await;
                        return Err(AppError::BadRequest("视频大小不能超过 500MB".to_string()));
                    }

                    file.write_all(&chunk).await?;
                }

                if size == 0 {
                    let _ = tokio::fs::remove_file(&target_path).await;
                    return Err(AppError::BadRequest("视频文件不能为空".to_string()));
                }

                stored_video_url = Some(format!("/uploads/videos/{}", stored_name));
                saved_path = Some(target_path);
            }
            _ => {}
        }
    }

    let Some(title) = title else {
        cleanup_saved_file(saved_path).await;
        return Err(AppError::BadRequest("标题不能为空".to_string()));
    };

    if title.len() > 128 {
        cleanup_saved_file(saved_path).await;
        return Err(AppError::BadRequest("标题不能超过 128 个字符".to_string()));
    }

    let Some(video_url) = stored_video_url else {
        cleanup_saved_file(saved_path).await;
        return Err(AppError::BadRequest("请上传视频文件".to_string()));
    };

    let result =
        sqlx::query("INSERT INTO videos (user_id, title, description, video_url) VALUES (?, ?, ?, ?)")
            .bind(user.id)
            .bind(&title)
            .bind(&description)
            .bind(&video_url)
            .execute(&state.pool)
            .await?;

    let video_id = result.last_insert_id() as i64;

    let video = fetch_video_by_id(&state, video_id, user.id).await?;

    Ok(Json(ApiResponse::ok(video)))
}

pub async fn list_my_videos(
    State(state): State<AppState>,
    Extension(user): Extension<AuthUser>,
    Query(query): Query<PageQuery>,
) -> Result<Json<ApiResponse<PageResponse<VideoDto>>>, AppError> {
    let page = query.page.unwrap_or(1).max(1);
    let page_size = query.page_size.unwrap_or(10).clamp(1, 50);
    let offset = ((page - 1) * page_size) as i64;

    let total = sqlx::query_as::<_, (i64,)>(
        "SELECT COUNT(*) FROM videos WHERE user_id = ? AND status = 1",
    )
    .bind(user.id)
    .fetch_one(&state.pool)
    .await?
    .0;

    let rows = sqlx::query_as::<_, VideoRow>(
        r#"
        SELECT
          v.id,
          v.user_id,
          u.username AS author,
          v.title,
          v.description,
          v.video_url,
          v.like_count,
          (
            SELECT COUNT(*)
            FROM video_likes vl
            WHERE vl.user_id = ? AND vl.video_id = v.id
          ) AS liked_count,
          v.created_at
        FROM videos v
        JOIN users u ON u.id = v.user_id
        WHERE v.user_id = ? AND v.status = 1
        ORDER BY v.created_at DESC
        LIMIT ? OFFSET ?
        "#,
    )
    .bind(user.id)
    .bind(user.id)
    .bind(page_size as i64)
    .bind(offset)
    .fetch_all(&state.pool)
    .await?;

    let items = rows
        .into_iter()
        .map(|row| row.into_dto(&state.config.public_base_url))
        .collect();

    Ok(Json(ApiResponse::ok(PageResponse {
        items,
        page,
        page_size,
        total,
    })))
}

pub async fn delete_my_video(
    State(state): State<AppState>,
    Extension(user): Extension<AuthUser>,
    AxumPath(video_id): AxumPath<i64>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    let video = sqlx::query_as::<_, VideoOwnerRow>(
        "SELECT user_id, video_url FROM videos WHERE id = ? AND status = 1",
    )
    .bind(video_id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or(AppError::NotFound)?;

    if video.user_id != user.id {
        return Err(AppError::Forbidden);
    }

    let file_path = video_file_path(&state.config.upload_dir, &video.video_url)?;

    if tokio::fs::try_exists(&file_path).await? {
        tokio::fs::remove_file(&file_path).await?;
    }

    sqlx::query("DELETE FROM videos WHERE id = ? AND user_id = ?")
        .bind(video_id)
        .bind(user.id)
        .execute(&state.pool)
        .await?;

    Ok(Json(ApiResponse::message("删除成功")))
}

async fn ensure_video_exists(state: &AppState, video_id: i64) -> Result<(), AppError> {
    let exists = sqlx::query_as::<_, (i64,)>("SELECT id FROM videos WHERE id = ? AND status = 1")
        .bind(video_id)
        .fetch_optional(&state.pool)
        .await?;

    exists.map(|_| ()).ok_or(AppError::NotFound)
}

async fn fetch_like_count(
    tx: &mut sqlx::Transaction<'_, sqlx::MySql>,
    video_id: i64,
) -> Result<i64, AppError> {
    Ok(sqlx::query_as::<_, (i64,)>("SELECT like_count FROM videos WHERE id = ?")
        .bind(video_id)
        .fetch_one(&mut **tx)
        .await?
        .0)
}

async fn fetch_video_by_id(
    state: &AppState,
    video_id: i64,
    current_user_id: i64,
) -> Result<VideoDto, AppError> {
    let video = sqlx::query_as::<_, VideoRow>(
        r#"
        SELECT
          v.id,
          v.user_id,
          u.username AS author,
          v.title,
          v.description,
          v.video_url,
          v.like_count,
          (
            SELECT COUNT(*)
            FROM video_likes vl
            WHERE vl.user_id = ? AND vl.video_id = v.id
          ) AS liked_count,
          v.created_at
        FROM videos v
        JOIN users u ON u.id = v.user_id
        WHERE v.id = ? AND v.status = 1
        "#,
    )
    .bind(current_user_id)
    .bind(video_id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or(AppError::NotFound)?;

    Ok(video.into_dto(&state.config.public_base_url))
}

async fn cleanup_saved_file(path: Option<PathBuf>) {
    if let Some(path) = path {
        let _ = tokio::fs::remove_file(path).await;
    }
}

fn video_file_path(upload_dir: &str, video_url: &str) -> Result<PathBuf, AppError> {
    let filename = Path::new(video_url)
        .file_name()
        .ok_or_else(|| AppError::BadRequest("视频路径无效".to_string()))?;

    Ok(Path::new(upload_dir).join(filename))
}
