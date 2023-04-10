#![doc= include_str!("../Readme.md")]

mod parser;
pub mod sections;
pub mod config;

pub use parser::parse_sections;
pub use sections::Section;
pub use config::Config;
