// https://www.youtube.com/watch?v=rcZSOLAI1lM&list=TLPQMDMwMTIwMjSd_tUmkn098A&index=1

use axum::{response::Html, routing::get, Router};
use askama::Template;
// use tokio::sync::Mutex;

mod templates;


/*
lazy_static::lazy_static! {
    static ref TIMINGS: Mutex<Vec<Timing>>  = Mutex::new(Vec::new());
}
*/

async fn index() -> Html<String> {
    return Html(
        templates::Index {
            timings: vec![
                templates::Timing { 
                    timing_type: "Activity", 
                    start: 0, 
                    stop: 56, 
                    id: 2 
                },
                templates::Timing { 
                    timing_type: "Activity", 
                    start: 45, 
                    stop: 70, 
                    id: 4 
                }
            ],
        }.render().unwrap()
    );
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(index));

    axum::Server::bind(&"0.0.0.0:41000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}