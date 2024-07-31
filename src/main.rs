use std::{
    collections::HashMap,
    env,
    net::{Ipv4Addr, SocketAddrV4},
};

use axum::{
    extract::Request,
    http::{header::CONTENT_TYPE, HeaderValue},
    response::IntoResponse,
    routing::get,
    Router,
};
use tokio::net::TcpListener;

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

    let port = env::var("PORT")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(3000u16);

    let listener = TcpListener::bind(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
