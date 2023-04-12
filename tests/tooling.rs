use std::fs;
use std::path::PathBuf;

use color_eyre::eyre::Context;
use haproxy_config::{parse_sections, Config};

fn read_config() -> color_eyre::Result<Config> {
    let path = "tests/minimal_haproxy.cfg";
    let file = fs::read_to_string(path).wrap_err("Could not open file")?;
    let lines = parse_sections(&file)
        .map_err(|e| e.with_path(PathBuf::from(path)))
        .wrap_err("Could not parse file")?;
    Config::try_from(lines.as_slice()).wrap_err("Could not build config from sections")
}

#[test]
fn parse_error_can_be_wrapped() {
    read_config().wrap_err("Could not read config").unwrap();
}
