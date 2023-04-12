use haproxy_config::{parse_sections, sections::borrowed::Section};

fn run_test(file: &str, path: &str) {
    let sections = parse_sections(file).map_err(|e| e.with_path(path)).unwrap();
    dbg!(&sections);

    assert!(sections
        .iter()
        .any(|s| matches!(s, Section::UnknownLine { .. })));
}

macro_rules! test_file {
    ($name:ident) => {
        #[test]
        fn $name() {
            let file = include_str!(std::concat!("unsupported/", stringify!($name), ".cfg"));
            let path = std::concat!(stringify!($name), "_haproxy.cfg");
            run_test(file, path);
        }
    };
}

// test_file! {conditional_blocks} // fails
test_file! {nonesens}
