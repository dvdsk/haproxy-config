use haproxy_config_parser::{parse_sections, ConfigSection};
use std::path::PathBuf;

macro_rules! test_file {
    ($name:ident) => {
        #[test]
        fn $name() {
            let file = include_str!(std::concat!("unsupported/", stringify!($name), ".cfg"));
            match parse_sections(file) {
                Err(e) => {
                    let path = std::concat!(stringify!($name), "_haproxy.cfg");
                    let e = e.with_path(PathBuf::from(path));
                    e.print();
                    panic!("{}", e.inner);
                }
                Ok(sections) => {
                    assert!(sections
                        .iter()
                        .any(|s| matches!(s, ConfigSection::UnknownLine { .. })))
                }
            }
        }
    };
}

test_file! {conditional_blocks}
