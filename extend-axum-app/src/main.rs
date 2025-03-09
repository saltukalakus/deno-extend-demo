use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use std::{net::SocketAddr};

use tower_http::services::ServeDir;


#[tokio::main]
async fn main() {
    // Build our application with a route
    let app = Router::new()
        .route("/", get(index))
        .nest_service("/static", ServeDir::new("static"));

    // Run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    println!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn index() -> impl IntoResponse {
    Html(include_str!("../static/index.html"))
}