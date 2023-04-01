use haproxy_config_parser::parse;
use std::path::PathBuf;

macro_rules! test_file {
    ($name:ident) => {
        #[test]
        fn $name() {
            let file = include_str!(std::concat!(stringify!($name), "_haproxy.cfg"));
            if let Err(e) = parse(file) {
                let path = std::concat!(stringify!($name), "_haproxy.cfg");
                let e = e.with_path(PathBuf::from(path));
                e.print();
                panic!("{}", e.inner);
            }
        }
    };
}

test_file! {minimal}
test_file! {medium}
test_file! {large}
