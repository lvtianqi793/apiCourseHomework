package com.videoflow.controller;

import com.videoflow.dto.AuthDtos;
import com.videoflow.dto.VideoDto;
import com.videoflow.filter.JwtAuthFilter;
import com.videoflow.service.VideoService;
import com.videoflow.util.ApiResponse;
import org.springframework.web.bind.annotation.DeleteMapping;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PatchMapping;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestAttribute;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RequestParam;
import org.springframework.web.bind.annotation.RestController;
import org.springframework.web.multipart.MultipartFile;

import java.io.IOException;

@RestController
@RequestMapping("/api/my")
public class MyController {

    private final VideoService videoService;

    public MyController(VideoService videoService) {
        this.videoService = videoService;
    }

    @PostMapping("/videos")
    public ApiResponse<VideoDto> publishVideo(
            @RequestAttribute(JwtAuthFilter.AUTH_USER_ID_ATTR) long userId,
            @RequestParam("title") String title,
            @RequestParam(value = "description", required = false) String description,
            @RequestParam("file") MultipartFile file) throws IOException {
        return ApiResponse.ok(videoService.publishVideo(userId, title, description, file));
    }

    @GetMapping("/videos")
    public ApiResponse<AuthDtos.PageResponse<VideoDto>> listMyVideos(
            @RequestAttribute(JwtAuthFilter.AUTH_USER_ID_ATTR) long userId,
            @RequestParam(value = "page", required = false) Integer page,
            @RequestParam(value = "page_size", required = false) Integer pageSize) {
        return ApiResponse.ok(videoService.listMyVideos(userId, page, pageSize));
    }

    @PatchMapping("/videos/{id}")
    public ApiResponse<VideoDto> updateMyVideo(
            @RequestAttribute(JwtAuthFilter.AUTH_USER_ID_ATTR) long userId,
            @PathVariable long id,
            @RequestBody AuthDtos.UpdateVideoBody body) {
        return ApiResponse.ok(videoService.updateMyVideo(userId, id, body.title(), body.description()));
    }

    @DeleteMapping("/videos/{id}")
    public ApiResponse<Void> deleteMyVideo(
            @RequestAttribute(JwtAuthFilter.AUTH_USER_ID_ATTR) long userId,
            @PathVariable long id) throws IOException {
        videoService.deleteMyVideo(userId, id);
        return ApiResponse.message("删除成功");
    }

    @GetMapping("/favorites")
    public ApiResponse<AuthDtos.PageResponse<VideoDto>> listMyFavorites(
            @RequestAttribute(JwtAuthFilter.AUTH_USER_ID_ATTR) long userId,
            @RequestParam(value = "page", required = false) Integer page,
            @RequestParam(value = "page_size", required = false) Integer pageSize) {
        return ApiResponse.ok(videoService.listMyFavorites(userId, page, pageSize));
    }
}
