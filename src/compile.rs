use strum::Display;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Display)]
pub enum Complier
{
  GCC,
  Clang,
  MSVC,
  Intel,
  Rustc,
  Roslyn,
}
