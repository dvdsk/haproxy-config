use haproxy_parser;

#[test]
fn minimal() {
    let file = include_str!("minimal_haproxy.cfg");
    haproxy_parser::parse(file).unwrap();
}

#[test]
fn large() {
    let file = include_str!("large_haproxy.cfg");
    haproxy_parser::parse(file).unwrap();
}
