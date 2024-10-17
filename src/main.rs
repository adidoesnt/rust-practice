pub mod utils;

use utils::env::{get_env_var, init_env};

fn main() {
    init_env();

    let port: u16 = get_env_var("PORT", "12001").parse::<u16>().unwrap();
    println!("Read port from env: {}", port);
}
