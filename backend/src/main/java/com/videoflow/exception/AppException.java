package com.videoflow.exception;

public class AppException extends RuntimeException {

    private final int statusCode;

    public AppException(int statusCode, String message) {
        super(message);
        this.statusCode = statusCode;
    }

    public int getStatusCode() {
        return statusCode;
    }

    public static AppException badRequest(String message) {
        return new AppException(400, "请求参数错误：" + message);
    }

    public static AppException unauthorized() {
        return new AppException(401, "未登录或登录已过期");
    }

    public static AppException forbidden() {
        return new AppException(403, "没有权限执行该操作");
    }

    public static AppException notFound() {
        return new AppException(404, "资源不存在");
    }

    public static AppException internal(String message) {
        return new AppException(500, "内部服务错误：" + message);
    }
}
