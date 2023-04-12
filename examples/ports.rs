use std::env;
use std::fs::read_to_string;

use haproxy_config::{config, parse_sections};

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

    let frontend_ports = config.frontends.values().map(|f| f.bind.addr.port);
    let listen_ports = config.listen.values().map(|f| f.bind.addr.port);
    let ports: Vec<_> = frontend_ports.chain(listen_ports).collect();
    println!("ports bound to by haproxy: {ports:?}");
}
