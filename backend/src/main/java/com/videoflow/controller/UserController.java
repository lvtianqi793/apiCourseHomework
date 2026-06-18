package com.videoflow.controller;

import com.videoflow.filter.JwtAuthFilter;
import com.videoflow.service.VideoService;
import com.videoflow.util.ApiResponse;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.RequestAttribute;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

import java.util.Map;

@RestController
@RequestMapping("/api/users")
public class UserController {

    private final VideoService videoService;

    public UserController(VideoService videoService) {
        this.videoService = videoService;
    }

    @GetMapping("/{id}")
    public ApiResponse<Map<String, Object>> getUserProfile(
            @RequestAttribute(JwtAuthFilter.AUTH_USER_ID_ATTR) long currentUserId,
            @PathVariable long id) {
        return ApiResponse.ok(videoService.getUserProfile(currentUserId, id));
    }
}
