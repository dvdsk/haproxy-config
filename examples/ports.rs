use std::env;
use std::fs::read_to_string;

use haproxy_config_parser::{config, parse_sections};

fn main() {
    let path = env::args()
        .nth(1)
        .expect("need the path to the haproxy cfg file");
    let config = read_to_string(&path).expect("haproxy config file needs to be readable");
    let parsed = parse_sections(config.as_str())
        .map_err(|e| e.with_path(path))
        .unwrap();
    println!("{parsed:#?}");
    let config: config::Config = parsed.as_slice().try_into().unwrap();
    println!("{config:#?}");
}
