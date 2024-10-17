use crate::utils::logger::Logger;
use dotenv::dotenv;

pub fn init_env() {
    Logger::debug(&format!("Reading env variables..."));
    dotenv().ok();
}

pub fn get_env_var(key: &str, default: &str) -> String {
    let val = std::env::var(key).unwrap_or(default.to_string());
    val
}
