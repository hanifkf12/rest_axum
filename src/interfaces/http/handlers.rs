use crate::application::post_service::{CreatePostDto, PostService, UpdatePostDto};
use crate::domain::error::PostError;
use axum::{extract::Path, extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;
use std::sync::Arc;
use tracing::error;
use uuid::Uuid;

/// Application state shared across handlers.
#[derive(Clone)]
pub struct AppState {
    pub post_service: Arc<PostService>,
}

/// Health check endpoint.
pub async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, Json(serde_json::json!({"status": "ok"}))).into_response()
}

/// List all posts.
pub async fn list_posts(State(state): State<AppState>) -> impl IntoResponse {
    match state.post_service.get_all().await {
        Ok(posts) => (StatusCode::OK, Json(serde_json::json!(posts))).into_response(),
        Err(err) => map_post_error(err).into_response(),
    }
}

/// Get a single post by ID.
pub async fn get_post(Path(id): Path<Uuid>, State(state): State<AppState>) -> impl IntoResponse {
    match state.post_service.get_by_id(&id).await {
        Ok(post) => (StatusCode::OK, Json(post)).into_response(),
        Err(err) => map_post_error(err).into_response(),
    }
}

/// Create a new post.
pub async fn create_post(
    State(state): State<AppState>,
    Json(payload): Json<CreatePostDto>,
) -> impl IntoResponse {
    match state.post_service.create(payload).await {
        Ok(post) => (StatusCode::CREATED, Json(post)).into_response(),
        Err(err) => map_post_error(err).into_response(),
    }
}

/// Update an existing post.
pub async fn update_post(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
    Json(payload): Json<UpdatePostDto>,
) -> impl IntoResponse {
    match state.post_service.update(id, payload).await {
        Ok(post) => (StatusCode::OK, Json(post)).into_response(),
        Err(err) => map_post_error(err).into_response(),
    }
}

/// Delete a post.
pub async fn delete_post(Path(id): Path<Uuid>, State(state): State<AppState>) -> impl IntoResponse {
    match state.post_service.delete(id).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(err) => map_post_error(err).into_response(),
    }
}

/// Error response structure.
#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

/// Maps domain errors to HTTP responses.
fn map_post_error(err: PostError) -> impl IntoResponse {
    let status = match &err {
        PostError::NotFound => StatusCode::NOT_FOUND,
        PostError::InvalidInput(_) => StatusCode::BAD_REQUEST,
        PostError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
        PostError::Cache(_) => StatusCode::INTERNAL_SERVER_ERROR,
    };
    let body = Json(ErrorResponse {
        error: err.to_string(),
    });
    error!(%status, reason = %body.0.error, "post operation failed");
    (status, body)
}
