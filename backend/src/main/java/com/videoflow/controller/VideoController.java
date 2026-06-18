package com.videoflow.controller;

import com.videoflow.dto.AuthDtos;
import com.videoflow.dto.CommentDto;
import com.videoflow.dto.DanmakuDto;
import com.videoflow.dto.VideoDto;
import com.videoflow.filter.JwtAuthFilter;
import com.videoflow.service.VideoService;
import com.videoflow.util.ApiResponse;
import org.springframework.web.bind.annotation.DeleteMapping;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestAttribute;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RequestParam;
import org.springframework.web.bind.annotation.RestController;

import java.util.List;

@RestController
@RequestMapping("/api/videos")
public class VideoController {

    private final VideoService videoService;

    public VideoController(VideoService videoService) {
        this.videoService = videoService;
    }

    @GetMapping("/recommend/next")
    public ApiResponse<?> recommendNext(
            @RequestAttribute(JwtAuthFilter.AUTH_USER_ID_ATTR) long userId,
            @RequestParam(value = "current_id", required = false) Long currentId) {
        var video = videoService.recommendNext(userId, currentId);
        if (video.isPresent()) {
            return ApiResponse.ok(video.get());
        }
        return ApiResponse.message("暂无可推荐视频");
    }

    @GetMapping("/recommend/prev")
    public ApiResponse<?> recommendPrev(
            @RequestAttribute(JwtAuthFilter.AUTH_USER_ID_ATTR) long userId,
            @RequestParam(value = "current_id", required = false) Long currentId) {
        var video = videoService.recommendPrev(userId, currentId);
        if (video.isPresent()) {
            return ApiResponse.ok(video.get());
        }
        return ApiResponse.message("暂无可推荐视频");
    }

    @GetMapping("/by-uuid/{uuid}")
    public ApiResponse<VideoDto> getVideoByUuid(
            @RequestAttribute(JwtAuthFilter.AUTH_USER_ID_ATTR) long userId,
            @PathVariable String uuid) {
        return ApiResponse.ok(videoService.getVideoByUuid(userId, uuid));
    }

    @PostMapping("/{id}/like")
    public ApiResponse<AuthDtos.LikeResponse> likeVideo(
            @RequestAttribute(JwtAuthFilter.AUTH_USER_ID_ATTR) long userId,
            @PathVariable long id) {
        return ApiResponse.ok(videoService.likeVideo(userId, id));
    }

    @DeleteMapping("/{id}/like")
    public ApiResponse<AuthDtos.LikeResponse> unlikeVideo(
            @RequestAttribute(JwtAuthFilter.AUTH_USER_ID_ATTR) long userId,
            @PathVariable long id) {
        return ApiResponse.ok(videoService.unlikeVideo(userId, id));
    }

    @GetMapping("/{id}/comments")
    public ApiResponse<List<CommentDto>> listComments(@PathVariable long id) {
        return ApiResponse.ok(videoService.listComments(id));
    }

    @PostMapping("/{id}/comments")
    public ApiResponse<CommentDto> postComment(
            @RequestAttribute(JwtAuthFilter.AUTH_USER_ID_ATTR) long userId,
            @PathVariable long id,
            @RequestBody AuthDtos.CommentBody body) {
        return ApiResponse.ok(videoService.postComment(userId, id, body.content()));
    }

    @PostMapping("/{id}/favorite")
    public ApiResponse<AuthDtos.FavoriteResponse> favoriteVideo(
            @RequestAttribute(JwtAuthFilter.AUTH_USER_ID_ATTR) long userId,
            @PathVariable long id) {
        return ApiResponse.ok(videoService.favoriteVideo(userId, id));
    }

    @DeleteMapping("/{id}/favorite")
    public ApiResponse<AuthDtos.FavoriteResponse> unfavoriteVideo(
            @RequestAttribute(JwtAuthFilter.AUTH_USER_ID_ATTR) long userId,
            @PathVariable long id) {
        return ApiResponse.ok(videoService.unfavoriteVideo(userId, id));
    }

    @GetMapping("/{id}/danmaku")
    public ApiResponse<List<DanmakuDto>> listDanmaku(@PathVariable long id) {
        return ApiResponse.ok(videoService.listDanmaku(id));
    }

    @PostMapping("/{id}/danmaku")
    public ApiResponse<DanmakuDto> sendDanmaku(
            @RequestAttribute(JwtAuthFilter.AUTH_USER_ID_ATTR) long userId,
            @PathVariable long id,
            @RequestBody AuthDtos.DanmakuBody body) {
        return ApiResponse.ok(videoService.sendDanmaku(userId, id, body.content(), body.timestampSec()));
    }

    @PostMapping("/{id}/share")
    public ApiResponse<AuthDtos.ShareResponse> incrementShare(@PathVariable long id) {
        return ApiResponse.ok(videoService.incrementShare(id));
    }
}
