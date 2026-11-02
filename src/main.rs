use std::collections::HashMap;
use axum::{Json, Router, response::IntoResponse, routing::post};
use axum::body::{Bytes, HttpBody};
use axum::extract::{Path, Query};
use axum::http::HeaderMap;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/", post(echo));

    // listen globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server started successfully at 0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn echo(
    body: Bytes
) -> impl IntoResponse {
    body.into_response()
}
