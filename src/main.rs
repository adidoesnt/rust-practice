pub mod utils;
pub mod api;

use utils::{env::init_env, server::init_server};

#[tokio::main]
async fn main() {
    init_env();
    init_server().await;
}
