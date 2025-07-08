use std::borrow::Cow;
pub struct C;
impl super::Language for C {
    fn lang_name() -> Cow<'static, str> { Cow::Borrowed("C") }
}
