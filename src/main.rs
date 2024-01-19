mod templates;
mod db;
mod error;

use db::{get_timings, push_timing, clear_timings};
use error::AppError;
use templates::{Index, TimingType};
use axum::{routing::{get, post}, Router};
use tower_http::services::ServeDir;


async fn index() -> Result<Index, AppError> {
    return Ok(Index { timings: get_timings().await });
}

async fn movement() -> Result<Index, AppError> {
    push_timing(TimingType::Movement).await?;
    return Ok(Index { timings: get_timings().await });
}

async fn consume() -> Result<Index, AppError> {
    push_timing(TimingType::Consume).await?;
    return Ok(Index { timings: get_timings().await });
}

#[axum::debug_handler]
async fn clear() -> Result<Index, AppError> {
    clear_timings().await;
    return Ok(Index { timings: get_timings().await });
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let app: Router = Router::new()
        .route("/", get(index))
        .route("/movement", post(movement))
        .route("/consume", post(consume))
        .route("/clear", post(clear))
        .nest_service("/assets", ServeDir::new("assets"));

    axum::Server::bind(&"0.0.0.0:41000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}