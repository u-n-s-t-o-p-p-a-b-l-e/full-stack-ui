mod upload;

use axum::{
    routing::{get, post},
    response::{Html, IntoResponse},
    Router,
};
use std::fs;

use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

mod register;
use register::{
    show_register_form,
    process_registration 
};


#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index_page))
        .route("/register", get(show_register_form))
        .route("/register", post(process_registration))
        .route("/", get(show_upload_form))
        .route("/upload", axum::routing::post(upload_image))
        .nest_service("/css", ServeDir::new("src/css"))
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

async fn show_upload_form() -> impl IntoResponse {
    Html(include_str!("upload.html"))
}
