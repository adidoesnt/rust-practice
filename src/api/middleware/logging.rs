use axum::{
    body::Body,
    http::{Method, Request},
    middleware::Next,
    response::Response,
};
use chrono::{DateTime, Utc};
use colour::green;
use std::time::SystemTime;

pub async fn logging_middleware(request: Request<Body>, next: Next) -> Response {
    let method: Method = request.method().clone();
    let path: String = request.uri().path_and_query().unwrap().to_string();
    let start_time: SystemTime = SystemTime::now();
    let start_time: DateTime<Utc> = start_time.into();

    green!("[{}] ", start_time);
    println!("{} {}", method, path);

    let response: Response = next.run(request).await;
    response
}
