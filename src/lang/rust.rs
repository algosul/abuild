use std::borrow::Cow;
pub struct Rust;
impl super::Language for Rust {
    fn lang_name() -> Cow<'static, str> { Cow::Borrowed("Rust") }
}
