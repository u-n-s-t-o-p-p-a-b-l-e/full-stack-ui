use axum::{
    routing::{get, post},
    response::{Html, IntoResponse},
    Router,
};
use std::fs;

use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index_page))
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

pub async fn index_page() -> impl IntoResponse {
    let html_content = fs::read_to_string("src/index.html")
        .expect("Failed to read HTML file");
    Html(html_content)
}
