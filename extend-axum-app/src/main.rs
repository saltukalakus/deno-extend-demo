use axum::{
    extract::Json,
    response::{Html, IntoResponse},
    routing::get,
    http::StatusCode,
    Router,
};

use deno_core::{JsRuntime, RuntimeOptions};
use serde::Deserialize;
use std::{net::SocketAddr, path::PathBuf};
use tower_http::services::ServeDir;

#[derive(Deserialize)]
struct JsCode {
    code: String,
}

#[tokio::main]
async fn main() {
    // Build our application with routes
    let app = Router::new()
        .route("/", get(index))
        .route("/execute", axum::routing::post(execute_js))
        .nest_service("/static", ServeDir::new("static"));

    // Run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    println!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn index() -> impl IntoResponse {
    Html(include_str!("../static/index.html"))
}

async fn execute_js(Json(payload): Json<JsCode>) -> impl IntoResponse {

    let mut runtime = JsRuntime::new(RuntimeOptions {
        //extensions: vec![ext],
        ..Default::default()
    });

    let result = runtime.execute_script("<usage>", payload.code);

    match result {
        Ok(_) => (StatusCode::OK, "Script executed successfully".to_string()),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
    }
}