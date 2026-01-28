use axum::{routing::get, Router};
use std::sync::Arc;
use axum::extract::Extension;
use tower_service::Service;
use worker::*;
use serde_json;
use crate::infrastructure::in_memory_repo::InMemoryRepo;
use crate::domain::Video::UseCase::GetVideo::GetVideoUseCase;
use crate::domain::Video::Entity::VideoEntity::VideoId;

// pub mod infrastructure;
// pub mod domain;

fn router() -> Router {
    // composition root: 実装をここで生成して注入する
    let repo = Arc::new(InMemoryRepo::new());
    Router::new()
        .route("/", get(root))
        .route("/demo_videos", get(demo_videos))
        .layer(Extension(repo))
}

#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    _env: Env,
    _ctx: Context,
) -> Result<axum::http::Response<axum::body::Body>> {
    Ok(router().call(req).await?)
}

pub async fn root() -> &'static str {
    "Hello Axum!"
}

async fn demo_videos(
    Extension(repo): Extension<Arc<InMemoryRepo>>
) -> String {
    let uc = GetVideoUseCase::new(repo.clone());
    let id = VideoId::new();
    match uc.execute(&id).await {
        Ok(v) => serde_json::to_string(&v).unwrap_or_default(),
        Err(_) => "[]".to_string(),
    }
}