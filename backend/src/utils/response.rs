use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T>
where
    T: Serialize,
{
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T> ApiResponse<T>
where
    T: Serialize,
{
    pub fn ok(data: T) -> Self {
        Self {
            code: 0,
            message: "ok".to_string(),
            data: Some(data),
        }
    }

    pub fn message(message: impl Into<String>) -> Self {
        Self {
            code: 0,
            message: message.into(),
            data: None,
        }
    }
}
