use haproxy_config_parser::{parse_sections, Config};
use std::path::PathBuf;

macro_rules! test_file {
    ($name:ident) => {
        #[test]
        fn $name() {
            let file = include_str!(std::concat!(stringify!($name), "_haproxy.cfg"));
            let path = std::concat!(stringify!($name), "_haproxy.cfg");
            let lines = parse_sections(file)
                .map_err(|e| e.with_path(PathBuf::from(path)))
                .unwrap();
            println!("{lines:#?}");

            let config = Config::try_from(lines.as_slice()).unwrap();
            println!("{config:#?}");
        }
    };
}

test_file! {minimal}
test_file! {medium}
test_file! {large}
