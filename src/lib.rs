#![doc= include_str!("../Readme.md")]

mod parser;
/// Zero copy representation of config sections.
pub mod sections;
/// Stricter owned representation of an entire config that is easy to query.
pub mod config;

pub use parser::parse_sections;
pub use config::Config;
