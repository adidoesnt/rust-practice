pub mod utils;

use utils::{env::init_env, server::get_server_host};

fn main() {
    init_env();
    let _host: String = get_server_host();
}
