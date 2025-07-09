use std::fmt::Display;
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    UnknownArch { arch: String },
    UnknownVendor { vendor: String },
    UnknownOs { os: String },
    UnknownAbi { abi: String },
}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::UnknownArch { arch } => write!(f, "unknown arch: {arch}"),
            Error::UnknownVendor { vendor } => {
                write!(f, "unknown vendor: {vendor}")
            }
            Error::UnknownOs { os } => write!(f, "unknown os: {os}"),
            Error::UnknownAbi { abi } => write!(f, "unknown abi: {abi}"),
        }
    }
}
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::UnknownArch { .. } => None,
            Error::UnknownVendor { .. } => None,
            Error::UnknownOs { .. } => None,
            Error::UnknownAbi { .. } => None,
        }
    }
}
pub type Result<T> = std::result::Result<T, Error>;
/// custom target
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Custom {
    /// e.g. `x86_64-unknown-linux-gnu`
    Triple {
        arch:   String,
        vendor: String,
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
            Custom::Triple { arch, vendor, os, abi: None } => {
                write!(f, "{arch}-{vendor}-{os}")
            }
            Custom::Triple { arch, vendor, os, abi: Some(abi) } => {
                write!(f, "{arch}-{vendor}-{os}-{abi}")
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
    pub fn host() -> Result<Self> { Ok(Target::Host) }

    pub fn custom(
        arch: String, vendor: String, os: String, abi: Option<String>,
    ) -> Result<Self> {
        Ok(Target::Custom(Custom::Triple { arch, vendor, os, abi }))
    }
}
impl Targets {
    pub fn try_default() -> Result<Self> { Self::host() }

    pub fn host() -> Result<Self> {
        Ok(Targets { targets: vec![Target::host()?] })
    }

    pub fn builder() -> TargetsBuilder { TargetsBuilder { targets: vec![] } }
}
impl TargetsBuilder {
    pub fn host(&mut self) -> Result<&mut Self> {
        self.targets.push(Target::host()?);
        Ok(self)
    }

    pub fn custom(
        &mut self, arch: String, vendor: String, os: String,
        abi: Option<String>,
    ) -> Result<&mut Self> {
        self.targets.push(Target::custom(arch, vendor, os, abi)?);
        Ok(self)
    }

    pub fn with_mut_targets(
        &mut self, f: impl FnOnce(&mut Vec<Target>),
    ) -> &mut Self {
        f(&mut self.targets);
        self
    }

    pub fn build(&self) -> Result<Targets> {
        Ok(Targets { targets: self.targets.clone() })
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn targets() {
        assert_eq!(Targets::host(), Targets::builder().host().unwrap().build());
    }
}
