package com.videoflow.security;

import com.videoflow.config.AppProperties;
import com.videoflow.exception.AppException;
import io.jsonwebtoken.Claims;
import io.jsonwebtoken.Jwts;
import io.jsonwebtoken.security.Keys;
import org.springframework.stereotype.Service;

import javax.crypto.SecretKey;
import java.nio.charset.StandardCharsets;
import java.security.MessageDigest;
import java.time.Instant;
import java.time.temporal.ChronoUnit;
import java.util.Date;

@Service
public class JwtService {

    private final SecretKey secretKey;

    public JwtService(AppProperties appProperties) {
        this.secretKey = deriveKey(appProperties.getJwtSecret());
    }

    private static SecretKey deriveKey(String secret) {
        try {
            byte[] hash = MessageDigest.getInstance("SHA-256")
                    .digest(secret.getBytes(StandardCharsets.UTF_8));
            return Keys.hmacShaKeyFor(hash);
        } catch (Exception e) {
            throw new IllegalStateException("初始化 JWT 密钥失败", e);
        }
    }

    public String createToken(long userId, String username) {
        Instant exp = Instant.now().plus(7, ChronoUnit.DAYS);
        return Jwts.builder()
                .subject(String.valueOf(userId))
                .claim("username", username)
                .expiration(Date.from(exp))
                .signWith(secretKey)
                .compact();
    }

    public JwtClaims decodeToken(String token) {
        try {
            Claims claims = Jwts.parser()
                    .verifyWith(secretKey)
                    .build()
                    .parseSignedClaims(token)
                    .getPayload();

            long userId = parseUserId(claims);
            String username = claims.get("username", String.class);
            return new JwtClaims(userId, username);
        } catch (AppException e) {
            throw e;
        } catch (Exception e) {
            throw AppException.unauthorized();
        }
    }

    private long parseUserId(Claims claims) {
        String subject = claims.getSubject();
        if (subject != null && !subject.isBlank()) {
            return Long.parseLong(subject);
        }

        Object sub = claims.get("sub");
        if (sub instanceof Number number) {
            return number.longValue();
        }
        if (sub instanceof String text && !text.isBlank()) {
            return Long.parseLong(text);
        }
        throw AppException.unauthorized();
    }

    public record JwtClaims(long userId, String username) {}
}
