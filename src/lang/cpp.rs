use std::borrow::Cow;
pub struct CPP;
impl super::Language for CPP {
    fn lang_name() -> Cow<'static, str> { Cow::Borrowed("C++") }
}
