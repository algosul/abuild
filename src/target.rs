use std::fmt::Display;
/// custom target
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Custom {
    /// e.g. `x86_64-unknown-linux-gnu`
    Triple {
        arch:   String,
        verder: String,
        os:     String,
        abi:    Option<String>,
    },
}
/// target triple
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub enum Target {
    /// host target
    #[default]
    Host,
    Custom(Custom),
}
impl Display for Custom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Custom::Triple { arch, verder, os, abi: None } => {
                write!(f, "{arch}-{verder}-{os}")
            }
            Custom::Triple { arch, verder, os, abi: Some(abi) } => {
                write!(f, "{arch}-{verder}-{os}-{abi}")
            }
        }
    }
}
impl Display for Target {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Target::Host => write!(f, "host"),
            Target::Custom(custom) => {
                write!(f, "{custom}")
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Targets {
    targets: Vec<Target>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TargetsBuilder {
    targets: Vec<Target>,
}
impl Target {
    pub fn host() -> Self { Target::Host }

    pub fn custom(
        arch: String, verder: String, os: String, abi: Option<String>,
    ) -> Self {
        Target::Custom(Custom::Triple { arch, verder, os, abi })
    }
}
impl Targets {
    pub fn new() -> Self { Targets { targets: vec![] } }

    pub fn host() -> Self { Targets { targets: vec![Target::host()] } }

    pub fn custom(
        arch: String, verder: String, os: String, abi: Option<String>,
    ) -> Self {
        Targets { targets: vec![Target::custom(arch, verder, os, abi)] }
    }

    pub fn builder() -> TargetsBuilder { TargetsBuilder { targets: vec![] } }
}
impl TargetsBuilder {
    pub fn host(&mut self) -> &mut Self {
        self.targets.push(Target::host());
        self
    }

    pub fn custom(
        &mut self, arch: String, verder: String, os: String,
        abi: Option<String>,
    ) -> &mut Self {
        self.targets.push(Target::custom(arch, verder, os, abi));
        self
    }

    pub fn build(&self) -> Targets { Targets { targets: self.targets.clone() } }
}
impl Default for Targets {
    fn default() -> Self { Self::new() }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn targets() {
        assert_eq!(Targets::host(), Targets::builder().host().build());
    }
}
