use std::collections::HashMap;

use axum::{
    extract::Request,
    http::{header::CONTENT_TYPE, HeaderValue},
    response::IntoResponse,
    routing::get,
    Router,
};

async fn get_headers(req: Request) -> impl IntoResponse {
    let headers: HashMap<&str, &str> = req
        .headers()
        .iter()
        .map(|(k, v)| (k.as_str(), v.to_str().unwrap_or("(failed to parse)")))
        .collect();

    let mut res = serde_json::to_string_pretty(&headers)
        .expect("json serialize")
        .into_response();

    res.headers_mut()
        .insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    res
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello, user! Try `GET /headers`" }))
        .route("/headers", get(get_headers));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
