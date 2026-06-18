package com.videoflow.config;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.List;

public final class DotEnvLoader {

    private DotEnvLoader() {}

    public static void load() {
        Path envFile = Path.of(".env");
        if (!Files.exists(envFile)) {
            return;
        }

        try {
            List<String> lines = Files.readAllLines(envFile);
            for (String rawLine : lines) {
                String line = rawLine.trim();
                if (line.isEmpty() || line.startsWith("#")) {
                    continue;
                }

                int idx = line.indexOf('=');
                if (idx <= 0) {
                    continue;
                }

                String key = line.substring(0, idx).trim();
                String value = line.substring(idx + 1).trim();
                if (key.isEmpty()) {
                    continue;
                }

                if (System.getenv(key) == null && System.getProperty(key) == null) {
                    System.setProperty(key, value);
                }
            }
        } catch (IOException ignored) {
            // .env 读取失败时回退到 application.yml 默认值
        }
    }
}
