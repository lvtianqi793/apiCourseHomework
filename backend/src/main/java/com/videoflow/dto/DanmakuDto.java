package com.videoflow.dto;

public class DanmakuDto {

    private final long id;
    private final long userId;
    private final String author;
    private final String content;
    private final float timestampSec;

    public DanmakuDto(long id, long userId, String author, String content, float timestampSec) {
        this.id = id;
        this.userId = userId;
        this.author = author;
        this.content = content;
        this.timestampSec = timestampSec;
    }

    public static DanmakuDto fromRow(DanmakuRow row) {
        return new DanmakuDto(row.id(), row.userId(), row.author(), row.content(), row.timestampSec());
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

    public float getTimestampSec() {
        return timestampSec;
    }

    public record DanmakuRow(long id, long userId, String author, String content, float timestampSec) {}
}
