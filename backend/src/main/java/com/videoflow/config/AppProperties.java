package com.videoflow.config;

import org.springframework.boot.context.properties.ConfigurationProperties;

@ConfigurationProperties(prefix = "app")
public class AppProperties {

    private String jwtSecret = "please-change-this-secret";
    private String frontendOrigin = "http://localhost:5173";
    private String publicBaseUrl = "http://localhost:8080";
    private String uploadDir = "uploads/videos";
    private long maxVideoSizeMb = 500;

    public String getJwtSecret() {
        return jwtSecret;
    }

    public void setJwtSecret(String jwtSecret) {
        this.jwtSecret = jwtSecret;
    }

    public String getFrontendOrigin() {
        return frontendOrigin;
    }

    public void setFrontendOrigin(String frontendOrigin) {
        this.frontendOrigin = frontendOrigin;
    }

    public String getPublicBaseUrl() {
        if (publicBaseUrl.endsWith("/")) {
            return publicBaseUrl.substring(0, publicBaseUrl.length() - 1);
        }
        return publicBaseUrl;
    }

    public void setPublicBaseUrl(String publicBaseUrl) {
        this.publicBaseUrl = publicBaseUrl;
    }

    public String getUploadDir() {
        return uploadDir;
    }

    public void setUploadDir(String uploadDir) {
        this.uploadDir = uploadDir;
    }

    public long getMaxVideoSizeMb() {
        return maxVideoSizeMb;
    }

    public void setMaxVideoSizeMb(long maxVideoSizeMb) {
        this.maxVideoSizeMb = maxVideoSizeMb;
    }

    public long getMaxVideoSizeBytes() {
        return maxVideoSizeMb * 1024 * 1024;
    }
}
