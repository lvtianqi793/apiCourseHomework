package com.videoflow.dto;

import com.fasterxml.jackson.annotation.JsonProperty;

import java.util.List;

public class AuthDtos {

    public record RegisterRequest(String username, String password) {}

    public record LoginRequest(String username, String password) {}

    public record UserDto(long id, String username) {}

    public record LoginResponse(String token, UserDto user) {}

    public record CommentBody(String content) {}

    public record DanmakuBody(String content, @JsonProperty("timestamp_sec") float timestampSec) {}

    public record UpdateVideoBody(String title, String description) {}

    public record LikeResponse(boolean liked, @JsonProperty("like_count") long likeCount) {}

    public record FavoriteResponse(boolean favorited) {}

    public record ShareResponse(@JsonProperty("share_count") long shareCount) {}

    public record PageResponse<T>(List<T> items, int page, @JsonProperty("page_size") int pageSize, long total) {}

    public record UserResultDto(long id, String username, @JsonProperty("video_count") long videoCount) {}
}
