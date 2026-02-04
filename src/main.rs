use crate::application::post_service::PostService;
use crate::infrastructure::cached_post_repository::CachedPostRepository;
use crate::infrastructure::config::Config;
use crate::infrastructure::post_repository::PostRepositoryImpl;
use crate::interfaces::http::handlers::{
    AppState, create_post, delete_post, get_post, health_check, list_posts, update_post,
};
use anyhow::Result;
use axum::Router;
use axum::routing::{delete, get, post, put};
use redis::Client;
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

    // Load configuration
    let config = Config::from_env()?;
    info!("Configuration loaded successfully");

    // Database connection
    let pool = PgPool::connect(&config.database_url).await?;
    info!("Database connected successfully");

    // Redis connection
    let redis_client = Client::open(config.redis_url)?;
    let redis_conn = redis_client.get_multiplexed_async_connection().await?;
    info!("Redis connected successfully");

    // Dependency injection - Clean Architecture: Infrastructure depends on Domain
    let post_repository = Arc::new(PostRepositoryImpl::new(pool));
    let cached_post_repository = Arc::new(CachedPostRepository::new(post_repository, redis_conn));

    let post_service = Arc::new(PostService::new(cached_post_repository));

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

    let addr = SocketAddr::from(([127, 0, 0, 1], config.server_port));
    info!("Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
