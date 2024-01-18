// https://www.youtube.com/watch?v=rcZSOLAI1lM&list=TLPQMDMwMTIwMjSd_tUmkn098A&index=1
// https://youtu.be/FlYchvaK3TM?si=IdgQ6Tp_FiRMXu_v


mod templates;
mod db;
mod error;

use db::{get_timings, push_timing, clear_timings};
use error::AppError;
use templates::{Index, TimingType};
use axum::{response::Html, routing::get, Router};
use askama::Template;
// use tokio::sync::Mutex;

/*
lazy_static::lazy_static! {
    static ref TIMINGS: Mutex<Vec<Timing>>  = Mutex::new(Vec::new());
}
*/

async fn index() -> Result<Index, AppError> {
    return Ok(Index { timings: get_timings().await });
}

#[tokio::main]
async fn main() {
    let app: Router = Router::new()
        .route("/", get(index));

    axum::Server::bind(&"0.0.0.0:41000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}