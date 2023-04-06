mod parser;
pub mod error;
pub mod sections;
pub mod config;

pub use parser::parse_sections;
pub use sections::ConfigSection;
pub use config::Config;
pub use error::Error;
