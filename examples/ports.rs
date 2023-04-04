use std::env;
use std::fs::read_to_string;

use haproxy_config_parser::{parse_sections, config};

fn main() {
    let path = env::args()
        .nth(1)
        .expect("need the path to the haproxy cfg file");
    let config = read_to_string(&path).expect("haproxy config file needs to be readable");
    let parsed = match parse_sections(config.as_str()) {
        Ok(parsed) => parsed,
        Err(error) => {
            error.with_path(path).print();
            return;
        }
    };
    println!("{parsed:#?}");
    let config: config::Config = parsed.as_slice().try_into().unwrap();
    println!("{config:#?}");
}
