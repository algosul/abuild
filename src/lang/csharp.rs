use std::borrow::Cow;
pub struct CSharp;
impl super::Language for CSharp {
    fn lang_name() -> Cow<'static, str> { Cow::Borrowed("C#") }
}
