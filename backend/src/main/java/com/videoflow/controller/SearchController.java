package com.videoflow.controller;

import com.videoflow.filter.JwtAuthFilter;
import com.videoflow.service.VideoService;
import com.videoflow.util.ApiResponse;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RequestAttribute;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RequestParam;
import org.springframework.web.bind.annotation.RestController;

import java.util.Map;

@RestController
@RequestMapping("/api/search")
public class SearchController {

    private final VideoService videoService;

    public SearchController(VideoService videoService) {
        this.videoService = videoService;
    }

    @GetMapping
    public ApiResponse<Map<String, Object>> search(
            @RequestAttribute(JwtAuthFilter.AUTH_USER_ID_ATTR) long userId,
            @RequestParam("q") String q) {
        return ApiResponse.ok(videoService.search(userId, q));
    }
}
