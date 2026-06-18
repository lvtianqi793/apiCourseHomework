package com.videoflow.dto;

import java.time.LocalDateTime;
import java.time.format.DateTimeFormatter;

public class CommentDto {

    private static final DateTimeFormatter FORMATTER = DateTimeFormatter.ofPattern("yyyy-MM-dd HH:mm:ss");

    private final long id;
    private final long userId;
    private final String author;
    private final String content;
    private final String createdAt;

    public CommentDto(long id, long userId, String author, String content, String createdAt) {
        this.id = id;
        this.userId = userId;
        this.author = author;
        this.content = content;
        this.createdAt = createdAt;
    }

    public static CommentDto fromRow(CommentRow row) {
        return new CommentDto(
                row.id(),
                row.userId(),
                row.author(),
                row.content(),
                row.createdAt().plusHours(8).format(FORMATTER));
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

    public String getContent() {
        return content;
    }

    public String getCreatedAt() {
        return createdAt;
    }

    public record CommentRow(long id, long userId, String author, String content, LocalDateTime createdAt) {}
}
