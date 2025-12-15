use crate::application::post_service::PostService;
use crate::infrastructure::post_repository::PostRepositoryImpl;
use crate::interfaces::http::handlers::{
    AppState, create_post, delete_post, get_post, health_check, list_posts, update_post,
};
use anyhow::Result;
use axum::Router;
use axum::routing::{delete, get, post, put};
use dotenvy::dotenv;
use sqlx::PgPool;
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing::info;

mod application;
mod domain;
mod infrastructure;
mod interfaces;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    // Load .env file
    dotenv().ok();

    // Database connection
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await?;

    let post_repository = Arc::new(PostRepositoryImpl::new(pool));

    let post_service = Arc::new(PostService::new(post_repository));

    let app_state = AppState { post_service };

    let app = Router::new()
        .route("/", get(health_check))
        .route("/posts", post(create_post))
        .route("/posts", get(list_posts))
        .route("/posts/:id", get(get_post))
        .route("/posts/:id", put(update_post))
        .route("/posts/:id", delete(delete_post))
        .with_state(app_state)
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
