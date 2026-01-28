use axum::{routing::get, Router};
use tower_service::Service;
use worker::*;

fn router() -> Router {
    Router::new()
    .route("/", get(root))
    .route("/demo_videos", get(demo_videos))
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

pub async fn demo_videos() -> &'static str {
    include_str!("../docs/api/v1/demo_video/response.json")
}
