package com.videoflow.service;

import com.videoflow.config.AppProperties;
import com.videoflow.dto.AuthDtos;
import com.videoflow.dto.CommentDto;
import com.videoflow.dto.DanmakuDto;
import com.videoflow.dto.VideoDto;
import com.videoflow.exception.AppException;
import com.videoflow.mapper.RowMappers;
import org.springframework.jdbc.core.JdbcTemplate;
import org.springframework.jdbc.support.GeneratedKeyHolder;
import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;
import org.springframework.web.multipart.MultipartFile;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.sql.PreparedStatement;
import java.sql.Statement;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Optional;
import java.util.UUID;

@Service
public class VideoService {

    private static final String VIDEO_SELECT = """
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
            """;

    private final JdbcTemplate jdbcTemplate;
    private final AppProperties appProperties;

    public VideoService(JdbcTemplate jdbcTemplate, AppProperties appProperties) {
        this.jdbcTemplate = jdbcTemplate;
        this.appProperties = appProperties;
    }

    public Optional<VideoDto> recommendNext(long userId, Long currentId) {
        VideoDto.VideoRow video;
        if (currentId != null) {
            video = fetchNextVideoInCycle(userId, currentId);
            if (video == null) {
                video = fetchFirstVideoInCycle(userId);
            }
        } else {
            video = fetchFirstVideoInCycle(userId);
        }
        if (video == null) {
            return Optional.empty();
        }
        recordVideoView(video.id());
        return Optional.of(VideoDto.fromRow(video, appProperties.getPublicBaseUrl()));
    }

    public Optional<VideoDto> recommendPrev(long userId, Long currentId) {
        VideoDto.VideoRow video;
        if (currentId != null) {
            video = fetchPrevVideoInCycle(userId, currentId);
            if (video == null) {
                video = fetchLastVideoInCycle(userId);
            }
        } else {
            video = fetchLastVideoInCycle(userId);
        }
        if (video == null) {
            return Optional.empty();
        }
        recordVideoView(video.id());
        return Optional.of(VideoDto.fromRow(video, appProperties.getPublicBaseUrl()));
    }

    public VideoDto getVideoByUuid(long userId, String uuid) {
        String pattern = "%/" + uuid + ".mp4";
        VideoDto.VideoRow video = jdbcTemplate.query(
                VIDEO_SELECT + " WHERE v.video_url LIKE ? AND v.status = 1",
                RowMappers.VIDEO_ROW,
                userId,
                userId,
                pattern).stream().findFirst().orElse(null);
        if (video == null) {
            throw AppException.notFound();
        }
        return VideoDto.fromRow(video, appProperties.getPublicBaseUrl());
    }

    @Transactional
    public AuthDtos.LikeResponse likeVideo(long userId, long videoId) {
        ensureVideoExists(videoId);
        int inserted = jdbcTemplate.update(
                "INSERT IGNORE INTO video_likes (user_id, video_id) VALUES (?, ?)",
                userId,
                videoId);
        if (inserted > 0) {
            jdbcTemplate.update("UPDATE videos SET like_count = like_count + 1 WHERE id = ?", videoId);
        }
        long likeCount = fetchLikeCount(videoId);
        return new AuthDtos.LikeResponse(true, likeCount);
    }

    @Transactional
    public AuthDtos.LikeResponse unlikeVideo(long userId, long videoId) {
        ensureVideoExists(videoId);
        int deleted = jdbcTemplate.update(
                "DELETE FROM video_likes WHERE user_id = ? AND video_id = ?",
                userId,
                videoId);
        if (deleted > 0) {
            jdbcTemplate.update("UPDATE videos SET like_count = GREATEST(like_count - 1, 0) WHERE id = ?", videoId);
        }
        long likeCount = fetchLikeCount(videoId);
        return new AuthDtos.LikeResponse(false, likeCount);
    }

    public List<CommentDto> listComments(long videoId) {
        ensureVideoExists(videoId);
        return jdbcTemplate.query(
                """
                SELECT c.id, c.user_id, u.username AS author, c.content, c.created_at
                FROM video_comments c
                JOIN users u ON u.id = c.user_id
                WHERE c.video_id = ?
                ORDER BY c.created_at ASC
                """,
                RowMappers.COMMENT_ROW,
                videoId).stream().map(CommentDto::fromRow).toList();
    }

    public CommentDto postComment(long userId, long videoId, String content) {
        ensureVideoExists(videoId);
        String trimmed = content == null ? "" : content.trim();
        if (trimmed.isEmpty() || trimmed.length() > 500) {
            throw AppException.badRequest("评论内容长度须在 1–500 字符之间");
        }

        GeneratedKeyHolder keyHolder = new GeneratedKeyHolder();
        jdbcTemplate.update(connection -> {
            PreparedStatement ps = connection.prepareStatement(
                    "INSERT INTO video_comments (video_id, user_id, content) VALUES (?, ?, ?)",
                    Statement.RETURN_GENERATED_KEYS);
            ps.setLong(1, videoId);
            ps.setLong(2, userId);
            ps.setString(3, trimmed);
            return ps;
        }, keyHolder);

        long commentId = keyHolder.getKey().longValue();
        return jdbcTemplate.query(
                """
                SELECT c.id, c.user_id, u.username AS author, c.content, c.created_at
                FROM video_comments c
                JOIN users u ON u.id = c.user_id
                WHERE c.id = ?
                """,
                RowMappers.COMMENT_ROW,
                commentId).stream().map(CommentDto::fromRow).findFirst().orElseThrow();
    }

    public AuthDtos.FavoriteResponse favoriteVideo(long userId, long videoId) {
        ensureVideoExists(videoId);
        jdbcTemplate.update(
                "INSERT IGNORE INTO video_favorites (user_id, video_id) VALUES (?, ?)",
                userId,
                videoId);
        return new AuthDtos.FavoriteResponse(true);
    }

    public AuthDtos.FavoriteResponse unfavoriteVideo(long userId, long videoId) {
        ensureVideoExists(videoId);
        jdbcTemplate.update(
                "DELETE FROM video_favorites WHERE user_id = ? AND video_id = ?",
                userId,
                videoId);
        return new AuthDtos.FavoriteResponse(false);
    }

    public List<DanmakuDto> listDanmaku(long videoId) {
        ensureVideoExists(videoId);
        return jdbcTemplate.query(
                """
                SELECT d.id, d.user_id, u.username AS author, d.content, d.timestamp_sec
                FROM video_danmaku d
                JOIN users u ON u.id = d.user_id
                WHERE d.video_id = ?
                ORDER BY d.timestamp_sec ASC
                """,
                RowMappers.DANMAKU_ROW,
                videoId).stream().map(DanmakuDto::fromRow).toList();
    }

    public DanmakuDto sendDanmaku(long userId, long videoId, String content, float timestampSec) {
        ensureVideoExists(videoId);
        String trimmed = content == null ? "" : content.trim();
        if (trimmed.isEmpty() || trimmed.length() > 200) {
            throw AppException.badRequest("弹幕内容长度须在 1–200 字符之间");
        }
        if (timestampSec < 0) {
            throw AppException.badRequest("时间戳不能为负数");
        }

        GeneratedKeyHolder keyHolder = new GeneratedKeyHolder();
        jdbcTemplate.update(connection -> {
            PreparedStatement ps = connection.prepareStatement(
                    "INSERT INTO video_danmaku (video_id, user_id, content, timestamp_sec) VALUES (?, ?, ?, ?)",
                    Statement.RETURN_GENERATED_KEYS);
            ps.setLong(1, videoId);
            ps.setLong(2, userId);
            ps.setString(3, trimmed);
            ps.setFloat(4, timestampSec);
            return ps;
        }, keyHolder);

        long danmakuId = keyHolder.getKey().longValue();
        return jdbcTemplate.query(
                """
                SELECT d.id, d.user_id, u.username AS author, d.content, d.timestamp_sec
                FROM video_danmaku d
                JOIN users u ON u.id = d.user_id
                WHERE d.id = ?
                """,
                RowMappers.DANMAKU_ROW,
                danmakuId).stream().map(DanmakuDto::fromRow).findFirst().orElseThrow();
    }

    public AuthDtos.ShareResponse incrementShare(long videoId) {
        ensureVideoExists(videoId);
        jdbcTemplate.update("UPDATE videos SET share_count = share_count + 1 WHERE id = ?", videoId);
        Long shareCount = jdbcTemplate.queryForObject(
                "SELECT share_count FROM videos WHERE id = ?",
                Long.class,
                videoId);
        return new AuthDtos.ShareResponse(shareCount == null ? 0 : shareCount);
    }

    public VideoDto publishVideo(long userId, String title, String description, MultipartFile file) throws IOException {
        if (title == null || title.trim().isEmpty()) {
            throw AppException.badRequest("标题不能为空");
        }
        String finalTitle = title.trim();
        if (finalTitle.length() > 128) {
            throw AppException.badRequest("标题不能超过 128 个字符");
        }
        if (file == null || file.isEmpty()) {
            throw AppException.badRequest("请上传视频文件");
        }

        String originalName = file.getOriginalFilename() == null ? "" : file.getOriginalFilename().toLowerCase();
        String contentType = file.getContentType() == null ? "" : file.getContentType();
        if (!originalName.endsWith(".mp4") && !"video/mp4".equals(contentType)) {
            throw AppException.badRequest("只允许上传 mp4 视频文件");
        }
        if (file.getSize() > appProperties.getMaxVideoSizeBytes()) {
            throw AppException.badRequest("视频大小不能超过 500MB");
        }

        Path uploadDir = Path.of(appProperties.getUploadDir()).toAbsolutePath();
        Files.createDirectories(uploadDir);
        String storedName = UUID.randomUUID() + ".mp4";
        Path targetPath = uploadDir.resolve(storedName);
        try {
            file.transferTo(targetPath);
        } catch (IOException e) {
            Files.deleteIfExists(targetPath);
            throw e;
        }

        String videoUrl = "/uploads/videos/" + storedName;
        String desc = description == null || description.trim().isEmpty() ? null : description.trim();

        GeneratedKeyHolder keyHolder = new GeneratedKeyHolder();
        jdbcTemplate.update(connection -> {
            PreparedStatement ps = connection.prepareStatement(
                    "INSERT INTO videos (user_id, title, description, video_url) VALUES (?, ?, ?, ?)",
                    Statement.RETURN_GENERATED_KEYS);
            ps.setLong(1, userId);
            ps.setString(2, finalTitle);
            ps.setString(3, desc);
            ps.setString(4, videoUrl);
            return ps;
        }, keyHolder);

        long videoId = keyHolder.getKey().longValue();
        return fetchVideoById(videoId, userId);
    }

    public AuthDtos.PageResponse<VideoDto> listMyVideos(long userId, Integer page, Integer pageSize) {
        int p = page == null ? 1 : Math.max(page, 1);
        int ps = pageSize == null ? 10 : Math.min(Math.max(pageSize, 1), 50);
        long offset = (long) (p - 1) * ps;

        Long total = jdbcTemplate.queryForObject(
                "SELECT COUNT(*) FROM videos WHERE user_id = ? AND status = 1",
                Long.class,
                userId);
        List<VideoDto> items = jdbcTemplate.query(
                VIDEO_SELECT + " WHERE v.user_id = ? AND v.status = 1 ORDER BY v.created_at DESC LIMIT ? OFFSET ?",
                RowMappers.VIDEO_ROW,
                userId,
                userId,
                userId,
                ps,
                offset).stream().map(row -> VideoDto.fromRow(row, appProperties.getPublicBaseUrl())).toList();

        return new AuthDtos.PageResponse<>(items, p, ps, total == null ? 0 : total);
    }

    @Transactional
    public void deleteMyVideo(long userId, long videoId) throws IOException {
        VideoDto.VideoOwnerRow video = jdbcTemplate.query(
                "SELECT user_id, video_url FROM videos WHERE id = ? AND status = 1",
                RowMappers.VIDEO_OWNER_ROW,
                videoId).stream().findFirst().orElse(null);
        if (video == null) {
            throw AppException.notFound();
        }
        if (video.userId() != userId) {
            throw AppException.forbidden();
        }

        Path filePath = videoFilePath(video.videoUrl());
        if (Files.exists(filePath)) {
            Files.delete(filePath);
        }
        jdbcTemplate.update("DELETE FROM videos WHERE id = ? AND user_id = ?", videoId, userId);
    }

    public VideoDto updateMyVideo(long userId, long videoId, String title, String description) {
        VideoDto.VideoOwnerRow video = jdbcTemplate.query(
                "SELECT user_id, video_url FROM videos WHERE id = ? AND status = 1",
                RowMappers.VIDEO_OWNER_ROW,
                videoId).stream().findFirst().orElse(null);
        if (video == null) {
            throw AppException.notFound();
        }
        if (video.userId() != userId) {
            throw AppException.forbidden();
        }

        if (title != null) {
            String trimmed = title.trim();
            if (trimmed.isEmpty()) {
                throw AppException.badRequest("标题不能为空");
            }
            if (trimmed.length() > 128) {
                throw AppException.badRequest("标题不能超过 128 个字符");
            }
            title = trimmed;
        }
        if (description != null) {
            description = description.trim();
        }

        jdbcTemplate.update(
                "UPDATE videos SET title = COALESCE(?, title), description = COALESCE(?, description) WHERE id = ?",
                title,
                description,
                videoId);
        return fetchVideoById(videoId, userId);
    }

    public AuthDtos.PageResponse<VideoDto> listMyFavorites(long userId, Integer page, Integer pageSize) {
        int p = page == null ? 1 : Math.max(page, 1);
        int ps = pageSize == null ? 10 : Math.min(Math.max(pageSize, 1), 50);
        long offset = (long) (p - 1) * ps;

        Long total = jdbcTemplate.queryForObject(
                "SELECT COUNT(*) FROM video_favorites WHERE user_id = ?",
                Long.class,
                userId);
        List<VideoDto> items = jdbcTemplate.query(
                VIDEO_SELECT + """
                 JOIN video_favorites fav ON fav.video_id = v.id AND fav.user_id = ?
                 WHERE v.status = 1
                 ORDER BY fav.created_at DESC
                 LIMIT ? OFFSET ?
                """,
                RowMappers.VIDEO_ROW,
                userId,
                userId,
                userId,
                ps,
                offset).stream().map(row -> VideoDto.fromRow(row, appProperties.getPublicBaseUrl())).toList();

        return new AuthDtos.PageResponse<>(items, p, ps, total == null ? 0 : total);
    }

    public Map<String, Object> search(long userId, String q) {
        String keyword = q == null ? "" : q.trim();
        if (keyword.isEmpty()) {
            return Map.of("users", List.of(), "videos", List.of());
        }
        String pattern = "%" + keyword + "%";

        List<AuthDtos.UserResultDto> users = jdbcTemplate.query(
                """
                SELECT u.id, u.username, CAST(COUNT(v.id) AS SIGNED) AS video_count
                FROM users u
                LEFT JOIN videos v ON v.user_id = u.id AND v.status = 1
                WHERE u.username LIKE ?
                GROUP BY u.id, u.username
                ORDER BY video_count DESC
                LIMIT 10
                """,
                (rs, rowNum) -> new AuthDtos.UserResultDto(
                        rs.getLong("id"),
                        rs.getString("username"),
                        rs.getLong("video_count")),
                pattern);

        List<VideoDto> videos = jdbcTemplate.query(
                VIDEO_SELECT + """
                 WHERE (v.title LIKE ? OR v.description LIKE ?) AND v.status = 1
                 ORDER BY (v.like_count * 3 + v.view_count + favorite_count * 5) DESC
                 LIMIT 20
                """,
                RowMappers.VIDEO_ROW,
                userId,
                userId,
                pattern,
                pattern).stream().map(row -> VideoDto.fromRow(row, appProperties.getPublicBaseUrl())).toList();

        Map<String, Object> result = new HashMap<>();
        result.put("users", users);
        result.put("videos", videos);
        return result;
    }

    public Map<String, Object> getUserProfile(long currentUserId, long userId) {
        var profile = jdbcTemplate.query(
                "SELECT id, username FROM users WHERE id = ?",
                (rs, rowNum) -> new AuthDtos.UserDto(rs.getLong("id"), rs.getString("username")),
                userId).stream().findFirst().orElse(null);
        if (profile == null) {
            throw AppException.notFound();
        }

        List<VideoDto> videos = jdbcTemplate.query(
                VIDEO_SELECT + " WHERE v.user_id = ? AND v.status = 1 ORDER BY v.created_at DESC LIMIT ?",
                RowMappers.VIDEO_ROW,
                currentUserId,
                currentUserId,
                userId,
                20).stream().map(row -> VideoDto.fromRow(row, appProperties.getPublicBaseUrl())).toList();

        Map<String, Object> result = new HashMap<>();
        result.put("user", profile);
        result.put("videos", videos);
        return result;
    }

    private VideoDto fetchVideoById(long videoId, long currentUserId) {
        VideoDto.VideoRow video = jdbcTemplate.query(
                VIDEO_SELECT + " WHERE v.id = ? AND v.status = 1",
                RowMappers.VIDEO_ROW,
                currentUserId,
                currentUserId,
                videoId).stream().findFirst().orElse(null);
        if (video == null) {
            throw AppException.notFound();
        }
        return VideoDto.fromRow(video, appProperties.getPublicBaseUrl());
    }

    private void ensureVideoExists(long videoId) {
        Integer id = jdbcTemplate.query(
                "SELECT id FROM videos WHERE id = ? AND status = 1",
                rs -> rs.next() ? rs.getInt("id") : null,
                videoId);
        if (id == null) {
            throw AppException.notFound();
        }
    }

    private long fetchLikeCount(long videoId) {
        Long count = jdbcTemplate.queryForObject("SELECT like_count FROM videos WHERE id = ?", Long.class, videoId);
        return count == null ? 0 : count;
    }

    private void recordVideoView(long videoId) {
        jdbcTemplate.update("UPDATE videos SET view_count = view_count + 1 WHERE id = ?", videoId);
    }

    private Path videoFilePath(String videoUrl) {
        String filename = Path.of(videoUrl).getFileName().toString();
        if (filename.isEmpty()) {
            throw AppException.badRequest("视频路径无效");
        }
        return Path.of(appProperties.getUploadDir()).resolve(filename);
    }

    private VideoDto.VideoRow fetchFirstVideoInCycle(long currentUserId) {
        return jdbcTemplate.query(
                VIDEO_SELECT + """
                 WHERE v.status = 1
                 ORDER BY
                   (v.view_count + v.like_count*3 + (SELECT COUNT(*) FROM video_favorites WHERE video_id = v.id)*5)
                   / (CASE WHEN v.user_id = ? THEN 2 ELSE 1 END) DESC,
                   v.id DESC
                 LIMIT 1
                """,
                RowMappers.VIDEO_ROW,
                currentUserId,
                currentUserId,
                currentUserId).stream().findFirst().orElse(null);
    }

    private VideoDto.VideoRow fetchLastVideoInCycle(long currentUserId) {
        return jdbcTemplate.query(
                VIDEO_SELECT + """
                 WHERE v.status = 1
                 ORDER BY
                   (v.view_count + v.like_count*3 + (SELECT COUNT(*) FROM video_favorites WHERE video_id = v.id)*5)
                   / (CASE WHEN v.user_id = ? THEN 2 ELSE 1 END) ASC,
                   v.id ASC
                 LIMIT 1
                """,
                RowMappers.VIDEO_ROW,
                currentUserId,
                currentUserId,
                currentUserId).stream().findFirst().orElse(null);
    }

    private VideoDto.VideoRow fetchNextVideoInCycle(long currentUserId, long currentVideoId) {
        return jdbcTemplate.query(
                VIDEO_SELECT + """
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
                """,
                RowMappers.VIDEO_ROW,
                currentUserId,
                currentUserId,
                currentVideoId,
                currentUserId,
                currentUserId,
                currentUserId,
                currentUserId,
                currentUserId).stream().findFirst().orElse(null);
    }

    private VideoDto.VideoRow fetchPrevVideoInCycle(long currentUserId, long currentVideoId) {
        return jdbcTemplate.query(
                VIDEO_SELECT + """
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
                """,
                RowMappers.VIDEO_ROW,
                currentUserId,
                currentUserId,
                currentVideoId,
                currentUserId,
                currentUserId,
                currentUserId,
                currentUserId,
                currentUserId).stream().findFirst().orElse(null);
    }
}
