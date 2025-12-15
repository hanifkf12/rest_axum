use crate::application::post_service::PostService;
use crate::domain::{error::PostError, post::PostDto};
use axum::{Json, extract::Path, extract::State, http::StatusCode, response::IntoResponse};
use serde::Serialize;
use std::sync::Arc;
use tracing::error;
use uuid::Uuid;

#[derive(Clone)]
pub struct AppState {
    pub post_service: Arc<PostService>,
}

pub async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, Json(serde_json::json!({"status": "ok"}))).into_response()
}

pub async fn list_posts(State(state): State<AppState>) -> impl IntoResponse {
    match state.post_service.get_all().await {
        Ok(posts) => (StatusCode::OK, Json(serde_json::json!(posts))).into_response(),
        Err(err) => map_post_error(err).into_response(),
    }
}

pub async fn get_post(Path(id): Path<Uuid>, State(state): State<AppState>) -> impl IntoResponse {
    match state.post_service.get_by_id(&id).await {
        Ok(post) => (StatusCode::OK, Json(post)).into_response(),
        Err(err) => map_post_error(err).into_response(),
    }
}

pub async fn create_post(
    State(state): State<AppState>,
    Json(payload): Json<PostDto>,
) -> impl IntoResponse {
    match state.post_service.create(payload).await {
        Ok(post) => (StatusCode::CREATED, Json(post)).into_response(),
        Err(err) => map_post_error(err).into_response(),
    }
}

pub async fn update_post(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
    Json(payload): Json<PostDto>,
) -> impl IntoResponse {
    match state.post_service.update(id, payload).await {
        Ok(post) => (StatusCode::OK, Json(post)).into_response(),
        Err(err) => map_post_error(err).into_response(),
    }
}

pub async fn delete_post(Path(id): Path<Uuid>, State(state): State<AppState>) -> impl IntoResponse {
    match state.post_service.delete(id).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(err) => map_post_error(err).into_response(),
    }
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

fn map_post_error(err: PostError) -> impl IntoResponse {
    let status = match &err {
        PostError::NotFound => StatusCode::NOT_FOUND,
        PostError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
    };
    let body = Json(ErrorResponse {
        error: err.to_string(),
    });
    error!(%status, reason = %body.0.error, "post operation failed");
    (status, body)
}
