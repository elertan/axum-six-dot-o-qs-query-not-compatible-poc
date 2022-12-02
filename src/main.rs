use std::net::SocketAddr;

use axum::Router;
use axum::routing::get;
use serde::Deserialize;
use serde_qs::axum::QsQuery;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Deserialize)]
struct MyQuery {
    sequence: Vec<i32>,
}

async fn root(QsQuery(qs): QsQuery<MyQuery>) -> &'static str {
    "Hello, World!"
}