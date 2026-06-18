USE video_flow;

CREATE TABLE IF NOT EXISTS video_comments (
  id BIGINT PRIMARY KEY AUTO_INCREMENT,
  video_id BIGINT NOT NULL,
  user_id BIGINT NOT NULL,
  content TEXT NOT NULL,
  created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  CONSTRAINT fk_comments_video FOREIGN KEY (video_id) REFERENCES videos(id) ON DELETE CASCADE,
  CONSTRAINT fk_comments_user FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
  INDEX idx_comments_video_time (video_id, created_at)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE IF NOT EXISTS video_favorites (
  id BIGINT PRIMARY KEY AUTO_INCREMENT,
  user_id BIGINT NOT NULL,
  video_id BIGINT NOT NULL,
  created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  UNIQUE KEY uk_user_video_fav (user_id, video_id),
  CONSTRAINT fk_favorites_user FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
  CONSTRAINT fk_favorites_video FOREIGN KEY (video_id) REFERENCES videos(id) ON DELETE CASCADE,
  INDEX idx_favorites_user_time (user_id, created_at)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE IF NOT EXISTS video_danmaku (
  id BIGINT PRIMARY KEY AUTO_INCREMENT,
  video_id BIGINT NOT NULL,
  user_id BIGINT NOT NULL,
  content VARCHAR(200) NOT NULL,
  timestamp_sec FLOAT NOT NULL,
  created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  CONSTRAINT fk_danmaku_video FOREIGN KEY (video_id) REFERENCES videos(id) ON DELETE CASCADE,
  CONSTRAINT fk_danmaku_user FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
  INDEX idx_danmaku_video_ts (video_id, timestamp_sec)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
