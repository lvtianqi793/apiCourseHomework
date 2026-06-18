package com.videoflow.filter;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.databind.node.ArrayNode;
import com.fasterxml.jackson.databind.node.ObjectNode;
import com.videoflow.security.JwtService;
import jakarta.servlet.FilterChain;
import jakarta.servlet.ServletException;
import jakarta.servlet.http.HttpServletRequest;
import jakarta.servlet.http.HttpServletResponse;
import org.springframework.jdbc.core.JdbcTemplate;
import org.springframework.scheduling.annotation.Async;
import org.springframework.stereotype.Component;
import org.springframework.web.filter.OncePerRequestFilter;
import org.springframework.web.util.ContentCachingRequestWrapper;
import org.springframework.web.util.ContentCachingResponseWrapper;

import java.io.IOException;
import java.nio.charset.StandardCharsets;
import java.util.Iterator;
import java.util.Map;

@Component
public class RequestLoggingFilter extends OncePerRequestFilter {

    private static final int LOG_BODY_LIMIT = 1024 * 1024;
    private static final String MULTIPART_PLACEHOLDER = "<multipart/form-data omitted>";

    private final JwtService jwtService;
    private final ObjectMapper objectMapper;
    private final RequestLogWriter requestLogWriter;

    public RequestLoggingFilter(
            JwtService jwtService,
            ObjectMapper objectMapper,
            RequestLogWriter requestLogWriter) {
        this.jwtService = jwtService;
        this.objectMapper = objectMapper;
        this.requestLogWriter = requestLogWriter;
    }

    @Override
    protected void doFilterInternal(
            HttpServletRequest request,
            HttpServletResponse response,
            FilterChain filterChain) throws ServletException, IOException {
        long started = System.currentTimeMillis();
        String method = request.getMethod();
        String path = request.getRequestURI();
        Long userId = extractUserId(request);

        boolean isMultipart = request.getContentType() != null
                && request.getContentType().toLowerCase().contains("multipart/form-data");

        if (isMultipart) {
            ContentCachingResponseWrapper wrappedResponse = new ContentCachingResponseWrapper(response);
            try {
                filterChain.doFilter(request, wrappedResponse);
            } finally {
                writeLog(started, userId, method, path, MULTIPART_PLACEHOLDER, wrappedResponse);
            }
            return;
        }

        ContentCachingRequestWrapper wrappedRequest = new ContentCachingRequestWrapper(request);
        ContentCachingResponseWrapper wrappedResponse = new ContentCachingResponseWrapper(response);

        try {
            filterChain.doFilter(wrappedRequest, wrappedResponse);
        } finally {
            writeLog(
                    started,
                    userId,
                    method,
                    path,
                    sanitizeBody(wrappedRequest.getContentAsByteArray()),
                    wrappedResponse);
        }
    }

    private void writeLog(
            long started,
            Long userId,
            String method,
            String path,
            String requestBody,
            ContentCachingResponseWrapper wrappedResponse) throws IOException {
        int statusCode = wrappedResponse.getStatus();
        String responseBody = sanitizeBody(wrappedResponse.getContentAsByteArray());
        long durationMs = System.currentTimeMillis() - started;

        requestLogWriter.writeLog(userId, method, path, requestBody, responseBody, statusCode, durationMs);
        wrappedResponse.copyBodyToResponse();
    }

    private Long extractUserId(HttpServletRequest request) {
        String auth = request.getHeader("Authorization");
        if (auth == null || !auth.startsWith("Bearer ")) {
            return null;
        }
        try {
            return jwtService.decodeToken(auth.substring(7)).userId();
        } catch (Exception e) {
            return null;
        }
    }

    private String sanitizeBody(byte[] bytes) {
        if (bytes == null || bytes.length == 0) {
            return "";
        }
        int length = Math.min(bytes.length, LOG_BODY_LIMIT);
        String text = new String(bytes, 0, length, StandardCharsets.UTF_8).trim();
        if (text.isEmpty()) {
            return "";
        }
        try {
            JsonNode node = objectMapper.readTree(text);
            scrubJson(node);
            return objectMapper.writeValueAsString(node);
        } catch (Exception e) {
            return text.length() > LOG_BODY_LIMIT ? text.substring(0, LOG_BODY_LIMIT) : text;
        }
    }

    private void scrubJson(JsonNode node) {
        if (node instanceof ObjectNode objectNode) {
            Iterator<Map.Entry<String, JsonNode>> fields = objectNode.fields();
            while (fields.hasNext()) {
                Map.Entry<String, JsonNode> entry = fields.next();
                String key = entry.getKey().toLowerCase();
                if (key.contains("password") || key.contains("token")) {
                    objectNode.put(entry.getKey(), "***");
                } else {
                    scrubJson(entry.getValue());
                }
            }
        } else if (node instanceof ArrayNode arrayNode) {
            for (JsonNode item : arrayNode) {
                scrubJson(item);
            }
        }
    }

    @Component
    public static class RequestLogWriter {

        private final JdbcTemplate jdbcTemplate;

        public RequestLogWriter(JdbcTemplate jdbcTemplate) {
            this.jdbcTemplate = jdbcTemplate;
        }

        @Async
        public void writeLog(
                Long userId,
                String method,
                String path,
                String requestBody,
                String responseBody,
                int statusCode,
                long durationMs) {
            try {
                jdbcTemplate.update(
                        """
                        INSERT INTO request_logs
                          (user_id, method, path, request_body, response_body, status_code, duration_ms)
                        VALUES (?, ?, ?, ?, ?, ?, ?)
                        """,
                        userId,
                        method,
                        path,
                        requestBody.isEmpty() ? null : requestBody,
                        responseBody.isEmpty() ? null : responseBody,
                        statusCode,
                        durationMs);
            } catch (Exception ignored) {
                // 与 Rust 版一致：日志写入失败不影响主流程
            }
        }
    }
}
