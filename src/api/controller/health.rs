use axum::response::{IntoResponse, Response};

pub async fn get_health() -> impl IntoResponse {
    Response::builder()
        .status(200)
        .body("Server is healthy!".to_string())
        .unwrap()
}
