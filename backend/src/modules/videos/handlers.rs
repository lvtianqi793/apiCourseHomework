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
            CommentBody, CommentRow, DanmakuBody, DanmakuRow, FavoriteResponse, LikeResponse,
            PageQuery, PageResponse, PrevQuery, RecommendQuery, SearchQuery, UpdateVideoBody,
            UserProfileRow, UserResultDto, VideoDto, VideoOwnerRow, VideoRow,
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
          v.view_count,
          v.like_count,
          (SELECT COUNT(*) FROM video_likes vl WHERE vl.user_id = ? AND vl.video_id = v.id) AS liked_count,
          (SELECT COUNT(*) FROM video_comments vc WHERE vc.video_id = v.id) AS comment_count,
          (SELECT COUNT(*) FROM video_favorites vf2 WHERE vf2.video_id = v.id) AS favorite_count,
          (SELECT COUNT(*) FROM video_favorites vf WHERE vf.user_id = ? AND vf.video_id = v.id) AS favorited_count,
          v.share_count,
          v.created_at
        FROM videos v
        JOIN users u ON u.id = v.user_id
        WHERE v.status = 1
        ORDER BY
          (v.view_count + v.like_count*3 + (SELECT COUNT(*) FROM video_favorites WHERE video_id = v.id)*5)
          / (CASE WHEN v.user_id = ? THEN 2 ELSE 1 END) DESC,
          v.id DESC
        LIMIT 1
        "#,
    )
    .bind(current_user_id)
    .bind(current_user_id)
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
          v.view_count,
          v.like_count,
          (SELECT COUNT(*) FROM video_likes vl WHERE vl.user_id = ? AND vl.video_id = v.id) AS liked_count,
          (SELECT COUNT(*) FROM video_comments vc WHERE vc.video_id = v.id) AS comment_count,
          (SELECT COUNT(*) FROM video_favorites vf2 WHERE vf2.video_id = v.id) AS favorite_count,
          (SELECT COUNT(*) FROM video_favorites vf WHERE vf.user_id = ? AND vf.video_id = v.id) AS favorited_count,
          v.share_count,
          v.created_at
        FROM videos v
        JOIN users u ON u.id = v.user_id
        WHERE v.status = 1
        ORDER BY
          (v.view_count + v.like_count*3 + (SELECT COUNT(*) FROM video_favorites WHERE video_id = v.id)*5)
          / (CASE WHEN v.user_id = ? THEN 2 ELSE 1 END) ASC,
          v.id ASC
        LIMIT 1
        "#,
    )
    .bind(current_user_id)
    .bind(current_user_id)
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
          v.view_count,
          v.like_count,
          (SELECT COUNT(*) FROM video_likes vl WHERE vl.user_id = ? AND vl.video_id = v.id) AS liked_count,
          (SELECT COUNT(*) FROM video_comments vc WHERE vc.video_id = v.id) AS comment_count,
          (SELECT COUNT(*) FROM video_favorites vf2 WHERE vf2.video_id = v.id) AS favorite_count,
          (SELECT COUNT(*) FROM video_favorites vf WHERE vf.user_id = ? AND vf.video_id = v.id) AS favorited_count,
          v.share_count,
          v.created_at
        FROM videos v
        JOIN users u ON u.id = v.user_id
        JOIN videos c ON c.id = ? AND c.status = 1
        WHERE v.status = 1
          AND (
            (v.view_count + v.like_count*3 + (SELECT COUNT(*) FROM video_favorites WHERE video_id = v.id)*5) / (CASE WHEN v.user_id = ? THEN 2 ELSE 1 END)
            < (c.view_count + c.like_count*3 + (SELECT COUNT(*) FROM video_favorites WHERE video_id = c.id)*5) / (CASE WHEN c.user_id = ? THEN 2 ELSE 1 END)
            OR (
              (v.view_count + v.like_count*3 + (SELECT COUNT(*) FROM video_favorites WHERE video_id = v.id)*5) / (CASE WHEN v.user_id = ? THEN 2 ELSE 1 END)
              = (c.view_count + c.like_count*3 + (SELECT COUNT(*) FROM video_favorites WHERE video_id = c.id)*5) / (CASE WHEN c.user_id = ? THEN 2 ELSE 1 END)
              AND v.id < c.id
            )
          )
        ORDER BY
          (v.view_count + v.like_count*3 + (SELECT COUNT(*) FROM video_favorites WHERE video_id = v.id)*5) / (CASE WHEN v.user_id = ? THEN 2 ELSE 1 END) DESC,
          v.id DESC
        LIMIT 1
        "#,
    )
    .bind(current_user_id)
    .bind(current_user_id)
    .bind(current_video_id)
    .bind(current_user_id)
    .bind(current_user_id)
    .bind(current_user_id)
    .bind(current_user_id)
    .bind(current_user_id)
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
          v.view_count,
          v.like_count,
          (SELECT COUNT(*) FROM video_likes vl WHERE vl.user_id = ? AND vl.video_id = v.id) AS liked_count,
          (SELECT COUNT(*) FROM video_comments vc WHERE vc.video_id = v.id) AS comment_count,
          (SELECT COUNT(*) FROM video_favorites vf2 WHERE vf2.video_id = v.id) AS favorite_count,
          (SELECT COUNT(*) FROM video_favorites vf WHERE vf.user_id = ? AND vf.video_id = v.id) AS favorited_count,
          v.share_count,
          v.created_at
        FROM videos v
        JOIN users u ON u.id = v.user_id
        JOIN videos c ON c.id = ? AND c.status = 1
        WHERE v.status = 1
          AND (
            (v.view_count + v.like_count*3 + (SELECT COUNT(*) FROM video_favorites WHERE video_id = v.id)*5) / (CASE WHEN v.user_id = ? THEN 2 ELSE 1 END)
            > (c.view_count + c.like_count*3 + (SELECT COUNT(*) FROM video_favorites WHERE video_id = c.id)*5) / (CASE WHEN c.user_id = ? THEN 2 ELSE 1 END)
            OR (
              (v.view_count + v.like_count*3 + (SELECT COUNT(*) FROM video_favorites WHERE video_id = v.id)*5) / (CASE WHEN v.user_id = ? THEN 2 ELSE 1 END)
              = (c.view_count + c.like_count*3 + (SELECT COUNT(*) FROM video_favorites WHERE video_id = c.id)*5) / (CASE WHEN c.user_id = ? THEN 2 ELSE 1 END)
              AND v.id > c.id
            )
          )
        ORDER BY
          (v.view_count + v.like_count*3 + (SELECT COUNT(*) FROM video_favorites WHERE video_id = v.id)*5) / (CASE WHEN v.user_id = ? THEN 2 ELSE 1 END) ASC,
          v.id ASC
        LIMIT 1
        "#,
    )
    .bind(current_user_id)
    .bind(current_user_id)
    .bind(current_video_id)
    .bind(current_user_id)
    .bind(current_user_id)
    .bind(current_user_id)
    .bind(current_user_id)
    .bind(current_user_id)
    .fetch_optional(&state.pool)
    .await
    .map_err(AppError::from)
}

async fn record_video_view(
    state: &AppState,
    _current_user_id: i64,
    video_id: i64,
) -> Result<(), AppError> {
    sqlx::query("UPDATE videos SET view_count = view_count + 1 WHERE id = ?")
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
          v.view_count,
          v.like_count,
          (SELECT COUNT(*) FROM video_likes vl WHERE vl.user_id = ? AND vl.video_id = v.id) AS liked_count,
          (SELECT COUNT(*) FROM video_comments vc WHERE vc.video_id = v.id) AS comment_count,
          (SELECT COUNT(*) FROM video_favorites vf2 WHERE vf2.video_id = v.id) AS favorite_count,
          (SELECT COUNT(*) FROM video_favorites vf WHERE vf.user_id = ? AND vf.video_id = v.id) AS favorited_count,
          v.share_count,
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
          v.view_count,
          v.like_count,
          (SELECT COUNT(*) FROM video_likes vl WHERE vl.user_id = ? AND vl.video_id = v.id) AS liked_count,
          (SELECT COUNT(*) FROM video_comments vc WHERE vc.video_id = v.id) AS comment_count,
          (SELECT COUNT(*) FROM video_favorites vf2 WHERE vf2.video_id = v.id) AS favorite_count,
          (SELECT COUNT(*) FROM video_favorites vf WHERE vf.user_id = ? AND vf.video_id = v.id) AS favorited_count,
          v.share_count,
          v.created_at
        FROM videos v
        JOIN users u ON u.id = v.user_id
        WHERE v.id = ? AND v.status = 1
        "#,
    )
    .bind(current_user_id)
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

// ── Comments ──────────────────────────────────────────────────────────────────

pub async fn list_comments(
    State(state): State<AppState>,
    AxumPath(video_id): AxumPath<i64>,
) -> Result<Json<ApiResponse<Vec<crate::modules::videos::models::CommentDto>>>, AppError> {
    ensure_video_exists(&state, video_id).await?;

    let rows = sqlx::query_as::<_, CommentRow>(
        r#"
        SELECT c.id, c.user_id, u.username AS author, c.content, c.created_at
        FROM video_comments c
        JOIN users u ON u.id = c.user_id
        WHERE c.video_id = ?
        ORDER BY c.created_at ASC
        "#,
    )
    .bind(video_id)
    .fetch_all(&state.pool)
    .await?;

    let items = rows.into_iter().map(|r| r.into_dto()).collect();
    Ok(Json(ApiResponse::ok(items)))
}

pub async fn post_comment(
    State(state): State<AppState>,
    Extension(user): Extension<AuthUser>,
    AxumPath(video_id): AxumPath<i64>,
    Json(body): Json<CommentBody>,
) -> Result<Json<ApiResponse<crate::modules::videos::models::CommentDto>>, AppError> {
    ensure_video_exists(&state, video_id).await?;

    let content = body.content.trim().to_string();
    if content.is_empty() || content.len() > 500 {
        return Err(AppError::BadRequest("评论内容长度须在 1–500 字符之间".to_string()));
    }

    let result = sqlx::query(
        "INSERT INTO video_comments (video_id, user_id, content) VALUES (?, ?, ?)",
    )
    .bind(video_id)
    .bind(user.id)
    .bind(&content)
    .execute(&state.pool)
    .await?;

    let comment_id = result.last_insert_id() as i64;

    let row = sqlx::query_as::<_, CommentRow>(
        r#"
        SELECT c.id, c.user_id, u.username AS author, c.content, c.created_at
        FROM video_comments c
        JOIN users u ON u.id = c.user_id
        WHERE c.id = ?
        "#,
    )
    .bind(comment_id)
    .fetch_one(&state.pool)
    .await?;

    Ok(Json(ApiResponse::ok(row.into_dto())))
}

// ── Favorites ─────────────────────────────────────────────────────────────────

pub async fn favorite_video(
    State(state): State<AppState>,
    Extension(user): Extension<AuthUser>,
    AxumPath(video_id): AxumPath<i64>,
) -> Result<Json<ApiResponse<FavoriteResponse>>, AppError> {
    ensure_video_exists(&state, video_id).await?;

    sqlx::query("INSERT IGNORE INTO video_favorites (user_id, video_id) VALUES (?, ?)")
        .bind(user.id)
        .bind(video_id)
        .execute(&state.pool)
        .await?;

    Ok(Json(ApiResponse::ok(FavoriteResponse { favorited: true })))
}

pub async fn unfavorite_video(
    State(state): State<AppState>,
    Extension(user): Extension<AuthUser>,
    AxumPath(video_id): AxumPath<i64>,
) -> Result<Json<ApiResponse<FavoriteResponse>>, AppError> {
    ensure_video_exists(&state, video_id).await?;

    sqlx::query("DELETE FROM video_favorites WHERE user_id = ? AND video_id = ?")
        .bind(user.id)
        .bind(video_id)
        .execute(&state.pool)
        .await?;

    Ok(Json(ApiResponse::ok(FavoriteResponse { favorited: false })))
}

// ── Danmaku ───────────────────────────────────────────────────────────────────

pub async fn list_danmaku(
    State(state): State<AppState>,
    AxumPath(video_id): AxumPath<i64>,
) -> Result<Json<ApiResponse<Vec<crate::modules::videos::models::DanmakuDto>>>, AppError> {
    ensure_video_exists(&state, video_id).await?;

    let rows = sqlx::query_as::<_, DanmakuRow>(
        r#"
        SELECT d.id, d.user_id, u.username AS author, d.content, d.timestamp_sec
        FROM video_danmaku d
        JOIN users u ON u.id = d.user_id
        WHERE d.video_id = ?
        ORDER BY d.timestamp_sec ASC
        "#,
    )
    .bind(video_id)
    .fetch_all(&state.pool)
    .await?;

    let items = rows.into_iter().map(|r| r.into_dto()).collect();
    Ok(Json(ApiResponse::ok(items)))
}

pub async fn send_danmaku(
    State(state): State<AppState>,
    Extension(user): Extension<AuthUser>,
    AxumPath(video_id): AxumPath<i64>,
    Json(body): Json<DanmakuBody>,
) -> Result<Json<ApiResponse<crate::modules::videos::models::DanmakuDto>>, AppError> {
    ensure_video_exists(&state, video_id).await?;

    let content = body.content.trim().to_string();
    if content.is_empty() || content.len() > 200 {
        return Err(AppError::BadRequest("弹幕内容长度须在 1–200 字符之间".to_string()));
    }
    if body.timestamp_sec < 0.0 {
        return Err(AppError::BadRequest("时间戳不能为负数".to_string()));
    }

    let result = sqlx::query(
        "INSERT INTO video_danmaku (video_id, user_id, content, timestamp_sec) VALUES (?, ?, ?, ?)",
    )
    .bind(video_id)
    .bind(user.id)
    .bind(&content)
    .bind(body.timestamp_sec)
    .execute(&state.pool)
    .await?;

    let danmaku_id = result.last_insert_id() as i64;

    let row = sqlx::query_as::<_, DanmakuRow>(
        r#"
        SELECT d.id, d.user_id, u.username AS author, d.content, d.timestamp_sec
        FROM video_danmaku d
        JOIN users u ON u.id = d.user_id
        WHERE d.id = ?
        "#,
    )
    .bind(danmaku_id)
    .fetch_one(&state.pool)
    .await?;

    Ok(Json(ApiResponse::ok(row.into_dto())))
}

pub async fn update_my_video(
    State(state): State<AppState>,
    Extension(user): Extension<AuthUser>,
    AxumPath(video_id): AxumPath<i64>,
    Json(body): Json<UpdateVideoBody>,
) -> Result<Json<ApiResponse<VideoDto>>, AppError> {
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

    if let Some(ref title) = body.title {
        let title = title.trim();
        if title.is_empty() {
            return Err(AppError::BadRequest("标题不能为空".to_string()));
        }
        if title.len() > 128 {
            return Err(AppError::BadRequest("标题不能超过 128 个字符".to_string()));
        }
    }

    sqlx::query(
        "UPDATE videos SET title = COALESCE(?, title), description = COALESCE(?, description) WHERE id = ?",
    )
    .bind(body.title.as_deref().map(str::trim))
    .bind(body.description.as_deref().map(str::trim))
    .bind(video_id)
    .execute(&state.pool)
    .await?;

    let dto = fetch_video_by_id(&state, video_id, user.id).await?;
    Ok(Json(ApiResponse::ok(dto)))
}

pub async fn list_my_favorites(
    State(state): State<AppState>,
    Extension(user): Extension<AuthUser>,
    Query(query): Query<PageQuery>,
) -> Result<Json<ApiResponse<PageResponse<VideoDto>>>, AppError> {
    let page = query.page.unwrap_or(1).max(1);
    let page_size = query.page_size.unwrap_or(10).clamp(1, 50);
    let offset = ((page - 1) * page_size) as i64;

    let total = sqlx::query_as::<_, (i64,)>(
        "SELECT COUNT(*) FROM video_favorites WHERE user_id = ?",
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
          v.view_count,
          v.like_count,
          (SELECT COUNT(*) FROM video_likes vl WHERE vl.user_id = ? AND vl.video_id = v.id) AS liked_count,
          (SELECT COUNT(*) FROM video_comments vc WHERE vc.video_id = v.id) AS comment_count,
          (SELECT COUNT(*) FROM video_favorites vf2 WHERE vf2.video_id = v.id) AS favorite_count,
          (SELECT COUNT(*) FROM video_favorites vf WHERE vf.user_id = ? AND vf.video_id = v.id) AS favorited_count,
          v.share_count,
          v.created_at
        FROM videos v
        JOIN users u ON u.id = v.user_id
        JOIN video_favorites fav ON fav.video_id = v.id AND fav.user_id = ?
        WHERE v.status = 1
        ORDER BY fav.created_at DESC
        LIMIT ? OFFSET ?
        "#,
    )
    .bind(user.id)
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

pub async fn increment_share(
    State(state): State<AppState>,
    Extension(_user): Extension<AuthUser>,
    AxumPath(video_id): AxumPath<i64>,
) -> Result<Json<ApiResponse<serde_json::Value>>, AppError> {
    ensure_video_exists(&state, video_id).await?;

    sqlx::query("UPDATE videos SET share_count = share_count + 1 WHERE id = ?")
        .bind(video_id)
        .execute(&state.pool)
        .await?;

    let share_count = sqlx::query_as::<_, (i64,)>("SELECT share_count FROM videos WHERE id = ?")
        .bind(video_id)
        .fetch_one(&state.pool)
        .await?
        .0;

    Ok(Json(ApiResponse::ok(serde_json::json!({ "share_count": share_count }))))
}

pub async fn get_video_by_uuid(
    State(state): State<AppState>,
    Extension(user): Extension<AuthUser>,
    AxumPath(uuid): AxumPath<String>,
) -> Result<Json<ApiResponse<VideoDto>>, AppError> {
    let pattern = format!("%/{}.mp4", uuid);
    let video = sqlx::query_as::<_, VideoRow>(
        r#"
        SELECT
          v.id,
          v.user_id,
          u.username AS author,
          v.title,
          v.description,
          v.video_url,
          v.view_count,
          v.like_count,
          (SELECT COUNT(*) FROM video_likes vl WHERE vl.user_id = ? AND vl.video_id = v.id) AS liked_count,
          (SELECT COUNT(*) FROM video_comments vc WHERE vc.video_id = v.id) AS comment_count,
          (SELECT COUNT(*) FROM video_favorites vf2 WHERE vf2.video_id = v.id) AS favorite_count,
          (SELECT COUNT(*) FROM video_favorites vf WHERE vf.user_id = ? AND vf.video_id = v.id) AS favorited_count,
          v.share_count,
          v.created_at
        FROM videos v
        JOIN users u ON u.id = v.user_id
        WHERE v.video_url LIKE ? AND v.status = 1
        "#,
    )
    .bind(user.id)
    .bind(user.id)
    .bind(&pattern)
    .fetch_optional(&state.pool)
    .await?
    .ok_or(AppError::NotFound)?;

    Ok(Json(ApiResponse::ok(video.into_dto(&state.config.public_base_url))))
}

// ── Search ────────────────────────────────────────────────────────────────────

pub async fn search(
    State(state): State<AppState>,
    Extension(user): Extension<AuthUser>,
    Query(query): Query<SearchQuery>,
) -> Result<Json<ApiResponse<serde_json::Value>>, AppError> {
    let q = query.q.trim().to_string();
    if q.is_empty() {
        return Ok(Json(ApiResponse::ok(serde_json::json!({ "users": [], "videos": [] }))));
    }
    let pattern = format!("%{}%", q);

    let user_rows = sqlx::query_as::<_, UserResultDto>(
        r#"
        SELECT u.id, u.username, CAST(COUNT(v.id) AS SIGNED) AS video_count
        FROM users u
        LEFT JOIN videos v ON v.user_id = u.id AND v.status = 1
        WHERE u.username LIKE ?
        GROUP BY u.id, u.username
        ORDER BY video_count DESC
        LIMIT 10
        "#,
    )
    .bind(&pattern)
    .fetch_all(&state.pool)
    .await?;

    let video_rows = sqlx::query_as::<_, VideoRow>(
        r#"
        SELECT
          v.id,
          v.user_id,
          u.username AS author,
          v.title,
          v.description,
          v.video_url,
          v.view_count,
          v.like_count,
          (SELECT COUNT(*) FROM video_likes vl WHERE vl.user_id = ? AND vl.video_id = v.id) AS liked_count,
          (SELECT COUNT(*) FROM video_comments vc WHERE vc.video_id = v.id) AS comment_count,
          (SELECT COUNT(*) FROM video_favorites vf2 WHERE vf2.video_id = v.id) AS favorite_count,
          (SELECT COUNT(*) FROM video_favorites vf WHERE vf.user_id = ? AND vf.video_id = v.id) AS favorited_count,
          v.share_count,
          v.created_at
        FROM videos v
        JOIN users u ON u.id = v.user_id
        WHERE (v.title LIKE ? OR v.description LIKE ?) AND v.status = 1
        ORDER BY (v.like_count * 3 + v.view_count + favorite_count * 5) DESC
        LIMIT 20
        "#,
    )
    .bind(user.id)
    .bind(user.id)
    .bind(&pattern)
    .bind(&pattern)
    .fetch_all(&state.pool)
    .await?;

    let videos: Vec<VideoDto> = video_rows
        .into_iter()
        .map(|r| r.into_dto(&state.config.public_base_url))
        .collect();

    Ok(Json(ApiResponse::ok(serde_json::json!({
        "users": user_rows,
        "videos": videos,
    }))))
}

// ── User profile ──────────────────────────────────────────────────────────────

pub async fn get_user_profile(
    State(state): State<AppState>,
    Extension(current_user): Extension<AuthUser>,
    AxumPath(user_id): AxumPath<i64>,
) -> Result<Json<ApiResponse<serde_json::Value>>, AppError> {
    let profile = sqlx::query_as::<_, UserProfileRow>(
        "SELECT id, username FROM users WHERE id = ?",
    )
    .bind(user_id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or(AppError::NotFound)?;

    let page_size: i64 = 20;

    let rows = sqlx::query_as::<_, VideoRow>(
        r#"
        SELECT
          v.id,
          v.user_id,
          u.username AS author,
          v.title,
          v.description,
          v.video_url,
          v.view_count,
          v.like_count,
          (SELECT COUNT(*) FROM video_likes vl WHERE vl.user_id = ? AND vl.video_id = v.id) AS liked_count,
          (SELECT COUNT(*) FROM video_comments vc WHERE vc.video_id = v.id) AS comment_count,
          (SELECT COUNT(*) FROM video_favorites vf2 WHERE vf2.video_id = v.id) AS favorite_count,
          (SELECT COUNT(*) FROM video_favorites vf WHERE vf.user_id = ? AND vf.video_id = v.id) AS favorited_count,
          v.share_count,
          v.created_at
        FROM videos v
        JOIN users u ON u.id = v.user_id
        WHERE v.user_id = ? AND v.status = 1
        ORDER BY v.created_at DESC
        LIMIT ?
        "#,
    )
    .bind(current_user.id)
    .bind(current_user.id)
    .bind(user_id)
    .bind(page_size)
    .fetch_all(&state.pool)
    .await?;

    let videos: Vec<VideoDto> = rows
        .into_iter()
        .map(|r| r.into_dto(&state.config.public_base_url))
        .collect();

    Ok(Json(ApiResponse::ok(serde_json::json!({
        "user": { "id": profile.id, "username": profile.username },
        "videos": videos
    }))))
}
