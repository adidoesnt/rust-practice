use crate::utils::logger::Logger;
use axum::{
    body::Body,
    http::{Method, Request},
    middleware::Next,
    response::Response,
};

pub async fn logging_middleware(request: Request<Body>, next: Next) -> Response {
    let method: Method = request.method().clone();
    let path: String = request.uri().path_and_query().unwrap().to_string();
    Logger::debug(&format!("{} {}", method, path));

    let response: Response = next.run(request).await;
    response
}
