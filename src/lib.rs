#![feature(associated_type_defaults)]
pub mod command;
pub mod compile;
pub mod workspace;
/// the app name
pub const NAME: &str = env!("CARGO_PKG_NAME");
/// the app version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
