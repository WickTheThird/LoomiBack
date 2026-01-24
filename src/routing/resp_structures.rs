use serde::Serialize;

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: HealthStatus,
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum HealthStatus {
    Ok,
    Error,
    Unavailable,
}


#[derive(Serialize)]
pub struct InvalidResponse {
    pub error: &'static str,
}


#[derive(Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ResponseCode {
    Ok,
    NotFound,
    NotImplemented,
    BadRequest,
    Unauthorized,
    Forbidden,
    InternalError,
    ServiceUnavailable,
}

// Helpers
#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub  success: bool,
    pub data: Option<T>,
    pub error: Option<&'static str>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(error_msg: &'static str) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error_msg),
        }
    }

    pub fn not_implemented() -> Self {
        Self {
            success: false,
            data: None,
            error: Some("Not implemented"),
        }
    }
}
