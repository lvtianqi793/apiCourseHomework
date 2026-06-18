package com.videoflow.service;

import com.videoflow.dto.AuthDtos;
import com.videoflow.exception.AppException;
import com.videoflow.security.JwtService;
import com.videoflow.security.PasswordService;
import org.springframework.jdbc.core.JdbcTemplate;
import org.springframework.stereotype.Service;

@Service
public class AuthService {

    private final JdbcTemplate jdbcTemplate;
    private final PasswordService passwordService;
    private final JwtService jwtService;

    public AuthService(
            JdbcTemplate jdbcTemplate,
            PasswordService passwordService,
            JwtService jwtService) {
        this.jdbcTemplate = jdbcTemplate;
        this.passwordService = passwordService;
        this.jwtService = jwtService;
    }

    public void register(AuthDtos.RegisterRequest payload) {
        String username = payload.username() == null ? "" : payload.username().trim();
        String rawPassword = payload.password() == null ? "" : payload.password().trim();

        if (username.length() < 3 || username.length() > 64) {
            throw AppException.badRequest("用户名长度需要在 3 到 64 个字符之间");
        }
        if (rawPassword.length() < 6 || rawPassword.length() > 64) {
            throw AppException.badRequest("密码长度需要在 6 到 64 个字符之间");
        }

        Integer exists = jdbcTemplate.query(
                "SELECT id FROM users WHERE username = ?",
                rs -> rs.next() ? rs.getInt("id") : null,
                username);
        if (exists != null) {
            throw AppException.badRequest("用户名已存在");
        }

        String passwordHash = passwordService.hashPassword(rawPassword);
        jdbcTemplate.update("INSERT INTO users (username, password_hash) VALUES (?, ?)", username, passwordHash);
    }

    public AuthDtos.LoginResponse login(AuthDtos.LoginRequest payload) {
        String username = payload.username() == null ? "" : payload.username().trim();

        var user = jdbcTemplate.query(
                "SELECT id, username, password_hash FROM users WHERE username = ?",
                rs -> {
                    if (!rs.next()) {
                        return null;
                    }
                    return new UserRow(rs.getLong("id"), rs.getString("username"), rs.getString("password_hash"));
                },
                username);

        if (user == null || !passwordService.verifyPassword(payload.password(), user.passwordHash())) {
            throw AppException.unauthorized();
        }

        String token = jwtService.createToken(user.id(), user.username());
        return new AuthDtos.LoginResponse(token, new AuthDtos.UserDto(user.id(), user.username()));
    }

    private record UserRow(long id, String username, String passwordHash) {}
}
