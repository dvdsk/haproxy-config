use std::env;
use std::fs::read_to_string;

use haproxy_config_parser::{config, parse_sections};

fn main() {
    let path = env::args()
        .nth(1)
        .expect("need the path to the haproxy cfg file");
    let config = read_to_string(&path).expect("haproxy config file needs to be readable");
    let sections = parse_sections(config.as_str())
        .map_err(|e| e.with_path(path))
        .unwrap();
    println!("{sections:#?}");
    let config: config::Config = sections.as_slice().try_into().unwrap();
    println!("{config:#?}");

    let frontend_addrs = config.frontends.values().map(|f| f.bind);
    let listen_addr = config.listen.values().map(|f| f.bind);
    let ports: Vec<_> = frontend_addrs.chain(listen_addr).map(|b| b.addr.port).collect();
}
