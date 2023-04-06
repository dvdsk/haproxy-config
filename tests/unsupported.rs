use haproxy_config_parser::{parse_sections, ConfigSection};
use std::path::PathBuf;

macro_rules! test_file {
    ($name:ident) => {
        #[test]
        fn $name() {
            let file = include_str!(std::concat!("unsupported/", stringify!($name), ".cfg"));
            let path = std::concat!(stringify!($name), "_haproxy.cfg");
            let sections = parse_sections(file)
                .map_err(|e| e.with_path(PathBuf::from(path)))
                .unwrap();

            assert!(sections
                .iter()
                .any(|s| matches!(s, ConfigSection::UnknownLine { .. })))
        }
    };
}

test_file! {conditional_blocks}
