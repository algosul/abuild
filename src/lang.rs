//! <details><summary>Languages</summary>
//!
//! * `"Rust"`
//! * `"C++"`
//! * `"C"`
//! * `"C#"`
//!
//! </details>
use std::borrow::Cow;
pub mod c;
pub mod cpp;
pub mod csharp;
pub mod rust;
pub trait Language {
    /// the name of the language
    ///
    /// e.g. `"C++"`, `"Rust"`
    fn lang_name() -> Cow<'static, str>;
}
