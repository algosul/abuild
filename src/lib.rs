#![feature(associated_type_defaults)]
//! Example
//! ```sh
//! $ abuild new -w my-workspace
//! ...
//! $ cd my-workspace
//! $ abuild new -j my-project -l rust
//! ...
//! $ abuild run
//! ...
//! Hello, world!
//! $ abuild clean
//! ...
//! ```
pub mod command;
pub mod feature;
pub mod lang;
pub mod module;
pub mod profile;
pub mod project;
pub mod target;
pub mod workspace;
/// the app name
pub const NAME: &str = env!("CARGO_PKG_NAME");
/// the app version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
