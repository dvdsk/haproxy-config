use std::env;
use std::fs::read_to_string;

use haproxy_config_parser::parse;

fn main() {
    let path = env::args()
        .nth(1)
        .expect("need the path to the haproxy cfg file");
    let config = read_to_string(&path).expect("haproxy config file needs to be readable");
    let parsed = match parse(config.as_str()) {
        Ok(parsed) => parsed,
        Err(error) => {
            error.with_path(path).print();
            return;
        }
    };
    println!("{parsed:#?}");
}
