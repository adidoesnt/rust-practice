use crate::utils::env::get_env_var;

pub fn get_server_host() -> String {
    let environment: String = get_env_var("ENV", "DEV");
    let protocol: &str = match environment.as_str() {
        "DEV" => "http",
        "PROD" => "https",
        _ => "http",
    };
    let port: u16 = get_env_var("PORT", "12001").parse::<u16>().unwrap();
    let hostname: String = get_env_var("HOST", "127.0.0.1");
    let host: String = format!("{}://{}:{}", protocol, hostname, port);
    println!("Resolved host: {}", host);
    host
}
