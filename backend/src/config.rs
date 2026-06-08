use std::env;

#[derive(Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub server_addr: String,
    pub frontend_origin: String,
    pub public_base_url: String,
    pub upload_dir: String,
    pub max_video_size_bytes: u64,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        let database_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| "mysql://root:password@localhost:3306/video_flow".to_string());
        let jwt_secret =
            env::var("JWT_SECRET").unwrap_or_else(|_| "dev-secret-please-change".to_string());
        let server_addr = env::var("SERVER_ADDR").unwrap_or_else(|_| "0.0.0.0:8080".to_string());
        let frontend_origin =
            env::var("FRONTEND_ORIGIN").unwrap_or_else(|_| "http://localhost:5173".to_string());
        let public_base_url =
            env::var("PUBLIC_BASE_URL").unwrap_or_else(|_| "http://localhost:8080".to_string());
        let upload_dir = env::var("UPLOAD_DIR").unwrap_or_else(|_| "uploads/videos".to_string());
        let max_video_size_mb = env::var("MAX_VIDEO_SIZE_MB")
            .ok()
            .and_then(|value| value.parse::<u64>().ok())
            .unwrap_or(500);

        Ok(Self {
            database_url,
            jwt_secret,
            server_addr,
            frontend_origin,
            public_base_url: public_base_url.trim_end_matches('/').to_string(),
            upload_dir,
            max_video_size_bytes: max_video_size_mb * 1024 * 1024,
        })
    }
}
