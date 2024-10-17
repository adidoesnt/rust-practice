use crate::utils::time::get_timestamp_utc;
use colour::{blue, green, red, yellow};

pub struct Logger;

impl Logger {
    pub fn info(msg: &String) {
        let now = self::get_timestamp_utc();
        green!("[{}] ", now);
        println!("{}", msg);
    }

    pub fn error(msg: &String) {
        let now = get_timestamp_utc();
        red!("[{}] ", now);
        println!("{}", msg);
    }

    pub fn debug(msg: &String) {
        let now = get_timestamp_utc();
        blue!("[{}] ", now);
        println!("{}", msg);
    }

    pub fn warn(msg: &String) {
        let now = get_timestamp_utc();
        yellow!("[{}] ", now);
        println!("{}", msg);
    }
}
