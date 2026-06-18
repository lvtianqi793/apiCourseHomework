package com.videoflow.dto;

import com.fasterxml.jackson.annotation.JsonProperty;

import java.time.LocalDateTime;
import java.time.format.DateTimeFormatter;

public class VideoDto {

    private static final DateTimeFormatter FORMATTER = DateTimeFormatter.ofPattern("yyyy-MM-dd HH:mm:ss");

    private final long id;
    @JsonProperty("user_id")
    private final long userId;
    private final String author;
    private final String title;
    private final String description;
    @JsonProperty("video_url")
    private final String videoUrl;
    @JsonProperty("view_count")
    private final long viewCount;
    @JsonProperty("like_count")
    private final long likeCount;
    private final boolean liked;
    @JsonProperty("comment_count")
    private final long commentCount;
    @JsonProperty("favorite_count")
    private final long favoriteCount;
    private final boolean favorited;
    @JsonProperty("share_count")
    private final long shareCount;
    @JsonProperty("created_at")
    private final String createdAt;

    public VideoDto(
            long id,
            long userId,
            String author,
            String title,
            String description,
            String videoUrl,
            long viewCount,
            long likeCount,
            boolean liked,
            long commentCount,
            long favoriteCount,
            boolean favorited,
            long shareCount,
            String createdAt) {
        this.id = id;
        this.userId = userId;
        this.author = author;
        this.title = title;
        this.description = description;
        this.videoUrl = videoUrl;
        this.viewCount = viewCount;
        this.likeCount = likeCount;
        this.liked = liked;
        this.commentCount = commentCount;
        this.favoriteCount = favoriteCount;
        this.favorited = favorited;
        this.shareCount = shareCount;
        this.createdAt = createdAt;
    }

    public static VideoDto fromRow(VideoRow row, String publicBaseUrl) {
        String videoUrl = row.videoUrl();
        if (!videoUrl.startsWith("http://") && !videoUrl.startsWith("https://")) {
            videoUrl = publicBaseUrl + videoUrl;
        }
        String createdAt = row.createdAt().plusHours(8).format(FORMATTER);
        return new VideoDto(
                row.id(),
                row.userId(),
                row.author(),
                row.title(),
                row.description(),
                videoUrl,
                row.viewCount(),
                row.likeCount(),
                row.likedCount() > 0,
                row.commentCount(),
                row.favoriteCount(),
                row.favoritedCount() > 0,
                row.shareCount(),
                createdAt);
    }

    public long getId() {
        return id;
    }

    public long getUserId() {
        return userId;
    }

    public String getAuthor() {
        return author;
    }

    public String getTitle() {
        return title;
    }

    public String getDescription() {
        return description;
    }

    public String getVideoUrl() {
        return videoUrl;
    }

    public long getViewCount() {
        return viewCount;
    }

    public long getLikeCount() {
        return likeCount;
    }

    public boolean isLiked() {
        return liked;
    }

    public long getCommentCount() {
        return commentCount;
    }

    public long getFavoriteCount() {
        return favoriteCount;
    }

    public boolean isFavorited() {
        return favorited;
    }

    public long getShareCount() {
        return shareCount;
    }

    public String getCreatedAt() {
        return createdAt;
    }

    public record VideoRow(
            long id,
            long userId,
            String author,
            String title,
            String description,
            String videoUrl,
            long viewCount,
            long likeCount,
            long likedCount,
            long commentCount,
            long favoriteCount,
            long favoritedCount,
            long shareCount,
            LocalDateTime createdAt) {}

    public record VideoOwnerRow(long userId, String videoUrl) {}
}
