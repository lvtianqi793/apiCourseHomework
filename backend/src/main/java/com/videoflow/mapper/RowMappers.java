package com.videoflow.mapper;

import com.videoflow.dto.CommentDto;
import com.videoflow.dto.DanmakuDto;
import com.videoflow.dto.VideoDto;
import org.springframework.jdbc.core.RowMapper;

import java.sql.ResultSet;
import java.sql.SQLException;
import java.sql.Timestamp;
import java.time.LocalDateTime;

public final class RowMappers {

    private RowMappers() {}

    public static final RowMapper<VideoDto.VideoRow> VIDEO_ROW = (rs, rowNum) -> new VideoDto.VideoRow(
            rs.getLong("id"),
            rs.getLong("user_id"),
            rs.getString("author"),
            rs.getString("title"),
            rs.getString("description"),
            rs.getString("video_url"),
            rs.getLong("view_count"),
            rs.getLong("like_count"),
            rs.getLong("liked_count"),
            rs.getLong("comment_count"),
            rs.getLong("favorite_count"),
            rs.getLong("favorited_count"),
            rs.getLong("share_count"),
            toLocalDateTime(rs.getTimestamp("created_at")));

    public static final RowMapper<VideoDto.VideoOwnerRow> VIDEO_OWNER_ROW = (rs, rowNum) ->
            new VideoDto.VideoOwnerRow(rs.getLong("user_id"), rs.getString("video_url"));

    public static final RowMapper<CommentDto.CommentRow> COMMENT_ROW = (rs, rowNum) ->
            new CommentDto.CommentRow(
                    rs.getLong("id"),
                    rs.getLong("user_id"),
                    rs.getString("author"),
                    rs.getString("content"),
                    toLocalDateTime(rs.getTimestamp("created_at")));

    public static final RowMapper<DanmakuDto.DanmakuRow> DANMAKU_ROW = (rs, rowNum) ->
            new DanmakuDto.DanmakuRow(
                    rs.getLong("id"),
                    rs.getLong("user_id"),
                    rs.getString("author"),
                    rs.getString("content"),
                    rs.getFloat("timestamp_sec"));

    private static LocalDateTime toLocalDateTime(Timestamp timestamp) {
        return timestamp == null ? null : timestamp.toLocalDateTime();
    }
}
