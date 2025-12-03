use std::sync::Arc;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use crate::application::post_service::PostService;

#[derive(Clone)]
pub struct AppState {
    pub post_service: Arc<PostService>
}

pub async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, Json(serde_json::json!({
        "status": "ok",
        "message": "Server is running"
    })))
}

pub async fn list_posts(
    State(state): State<AppState>,
) -> impl IntoResponse {
    match state.post_service.get_all().await { 
        Ok(posts) => (StatusCode::OK, Json(serde_json::json!(posts))).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}

