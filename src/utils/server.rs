use crate::{
    api::{controller::health, middleware::logging},
    utils::{env::get_env_var, logger::Logger},
};
use axum::{middleware, routing::get, Router};
use tokio::net::TcpListener;

fn get_server_host() -> String {
    let port: u16 = get_env_var("PORT", "12001").parse::<u16>().unwrap();
    let hostname: String = get_env_var("HOST", "127.0.0.1");
    let host: String = format!("{}:{}", hostname, port);
    Logger::info(&format!("Resolved host: {}", host));
    host
}

#[axum::debug_handler]
pub async fn init_server() {
    Logger::debug(&format!("Initialising server..."));
    let host: String = get_server_host();
    let app = Router::new()
        .route("/", get(health::get_health))
        .route("/health", get(health::get_health))
        .layer(middleware::from_fn(logging::logging_middleware));
    let listener: TcpListener = TcpListener::bind(&host).await.unwrap();
    Logger::info(&format!("Listening on {}", host));
    axum::serve(listener, app).await.unwrap();
}
