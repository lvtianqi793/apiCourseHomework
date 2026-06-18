package com.videoflow.controller;

import com.videoflow.dto.AuthDtos;
import com.videoflow.service.AuthService;
import com.videoflow.util.ApiResponse;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

@RestController
@RequestMapping("/api/auth")
public class AuthController {

    private final AuthService authService;

    public AuthController(AuthService authService) {
        this.authService = authService;
    }

    @PostMapping("/register")
    public ApiResponse<Void> register(@RequestBody AuthDtos.RegisterRequest payload) {
        authService.register(payload);
        return ApiResponse.message("注册成功");
    }

    @PostMapping("/login")
    public ApiResponse<AuthDtos.LoginResponse> login(@RequestBody AuthDtos.LoginRequest payload) {
        return ApiResponse.ok(authService.login(payload));
    }
}
