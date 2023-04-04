mod parser;
pub mod error;
pub mod lines;
pub mod config;

pub use parser::parse_sections;
pub use error::Error;
pub use config::Config;
