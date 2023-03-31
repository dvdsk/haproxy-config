use haproxy_config_parser::parse;

#[test]
fn minimal() {
    let file = include_str!("minimal_haproxy.cfg");
    if let Err(e) = parse(file) {
        e.print();
        println!("{}", e.inner);
        println!("{:?}", e.inner);
        unreachable!();
    }
}

// #[test]
// fn large() {
//     let file = include_str!("large_haproxy.cfg");
//     parse(file).unwrap();
// }
