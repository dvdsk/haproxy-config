# HAProxy config parser

> **Parse HAProxy configs and easily query it**

<!-- [![Crates.io](https://img.shields.io/crates/v/dbstruct?style=flat-square)](https://crates.io/crates/dbstruct) -->
<!-- [![Crates.io](https://img.shields.io/crates/d/dbstruct?style=flat-square)](https://crates.io/crates/dbstruct) -->
<!-- [![API](https://docs.rs/dbstruct/badge.svg)](https://docs.rs/dbstruct) -->
[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](LICENSE-MIT)

A parser for HAProxy config files. HAProxy's configs have many options to many to build a completely typed API. Such an API would also be quite fragile to changes in the config. This crate therefore presents a loosely typed config. 

It parses to sections consisting of [lines](sections::Line) from which a [Config](Config) struct can be made. The struct follows the sections of a HAProxy config. Most options within the sections are presented in a [HashMap](std::collections::HashMap) as key value strings. The important settings have a fully typed API.

### Example
List all the ports HAProxy will bind to from the config file.
```rust
use haproxy_config_parser::parse_sections;
use haproxy_config_parser::Config;

let file = include_str!("../tests/medium_haproxy.cfg");
let sections = parse_sections(file).unwrap();

let config = Config::try_from(&sections).unwrap();

let frontend_ports = config.frontends.values().map(|f| f.bind.addr.port);
let listen_ports = config.listen.values().map(|f| f.bind.addr.port);
let ports: Vec<_> = frontend_ports.chain(listen_ports).collect();

println!("ports bound to by haproxy: {ports:?}")
```

### Features
 - Zero copy parsing to a config section based representation which preserves the order of the config lines and any comments.
 - A stricter owned representation that is easy to query.
 - Clear error reporting on parsing errors featuring a debug formatter that shows the problem with the config being parsed.
 - Panic free implementation, this crate will not crash your code.

### Unsupported
Because the API is not fully typed the crate allows some invalid configs to parse. Specifically any invalid configuration inside a section will be interpreted as a configuration value. Sections end at the next section or the end of the file therefore any invalid configuration after the first section is undetected. 

The crate will break on valid configs that feature [conditional blocks](https://www.haproxy.com/documentation/hapee/latest/onepage/#2.4). 

### Contributions
This crate is far from complete but covers all my own use cases. I do however welcome any contributions.

### Thanks
- builds upon the PEG grammar written by [Joey](https://github.com/imjoey) for [pyhaproxy](https://github.com/imjoey/pyhaproxy)
