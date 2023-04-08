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

#[test]
fn system_user_and_group() {
    let file = include_str!("medium_haproxy.cfg");
    let path = "medium_haproxy.cfg";

    let lines = parse_sections(file)
        .map_err(|e| e.with_path(PathBuf::from(path)))
        .unwrap();
    let config = Config::try_from(lines.as_slice()).unwrap();

    assert_eq!(config.global.user, Some("haproxy".into()));
    assert_eq!(config.global.group, Some("haproxy".into()));
}
