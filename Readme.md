# HAProxy config parser

> **Parse HAProxy configs and easily query it

<!-- [![Crates.io](https://img.shields.io/crates/v/dbstruct?style=flat-square)](https://crates.io/crates/dbstruct) -->
<!-- [![Crates.io](https://img.shields.io/crates/d/dbstruct?style=flat-square)](https://crates.io/crates/dbstruct) -->
<!-- [![API](https://docs.rs/dbstruct/badge.svg)](https://docs.rs/dbstruct) -->
[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](LICENSE-MIT)

A parser for HAProxy config files. HAProxy's configs have many options to many to build a completely typed API. Such an API would also be quite fragile to changes in the config. This crate therefore presents a loosely typed config. 

It parses to sections consisting of [lines](parser::Line) from which a [Config] struct can be made. The struct follows the sections of a HAProxy config. Most options within the sections are presented in a [HashMap] as key value strings. The important settings have a fully typed API.

### Example
```rust
use haproxy_config_parser::parse_sections;
use haproxy_config_parser::Config;

let file = include_str!("../tests/medium_haproxy.cfg");
let sections = parse_sections(file).unwrap();

let config = Config::try_from(&sections).unwrap();
let ports = config.frontends.iter()
```

### Unsupported
[conditional blocks](https://www.haproxy.com/documentation/hapee/latest/onepage/#2.4)

invalid code inside a section will be interpreted as a config value

### Thanks
- adopts the PEG by @imjoey written for [pyhaproxy](https://github.com/imjoey/pyhaproxy)
