use serde::Serialize;

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
}


#[derive(Serialize)]
pub struct InvalidResponse {
    pub error: &'static str,
}
