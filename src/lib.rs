#![doc= include_str!("../Readme.md")]

mod parser;
/// Zero copy and Owned representations of config sections.
pub mod section;

/// Zero copy and Owned representations of the lines in a config section.
pub mod line;

/// Stricter owned representation of an entire config that is easy to query.
pub mod config;

pub use parser::parse_sections;
pub use config::Config;
