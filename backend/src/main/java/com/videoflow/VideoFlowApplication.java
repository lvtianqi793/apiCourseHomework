package com.videoflow;

import com.videoflow.config.AppProperties;
import com.videoflow.config.DotEnvLoader;
import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.boot.context.properties.EnableConfigurationProperties;
import org.springframework.scheduling.annotation.EnableAsync;

import java.nio.file.Files;
import java.nio.file.Path;

@SpringBootApplication
@EnableConfigurationProperties(AppProperties.class)
@EnableAsync
public class VideoFlowApplication {

    public static void main(String[] args) throws Exception {
        DotEnvLoader.load();
        var context = SpringApplication.run(VideoFlowApplication.class, args);
        var props = context.getBean(AppProperties.class);
        Files.createDirectories(Path.of(props.getUploadDir()));
    }
}
