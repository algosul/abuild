use std::fmt::Display;

use crate::{profile::Profiles, target::Targets};
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Error {
    NoName,
    NoVersion,
    NoProfiles,
    NoTargets,
}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::NoName => write!(f, "No name provided"),
            Error::NoVersion => write!(f, "No version provided"),
            Error::NoProfiles => write!(f, "No profiles provided"),
            Error::NoTargets => write!(f, "No targets provided"),
        }
    }
}
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> { None }
}
pub type Result<T> = std::result::Result<T, Error>;
#[derive(Debug, Clone, PartialEq)]
pub struct Project {
    name:     String,
    version:  String,
    profiles: Profiles,
    targets:  Targets,
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ProjectBuilder {
    name:     Option<String>,
    version:  Option<String>,
    profiles: Option<Profiles>,
    targets:  Option<Targets>,
}
impl Project {
    pub fn default() -> Result<Self> { Self::builder().build() }

    pub fn builder() -> ProjectBuilder { ProjectBuilder::default() }
}
impl ProjectBuilder {
    pub fn name(&mut self, name: impl Into<String>) -> &mut Self {
        self.name = Some(name.into());
        self
    }

    pub fn version(&mut self, version: impl Into<String>) -> &mut Self {
        self.version = Some(version.into());
        self
    }

    pub fn profiles(&mut self, profiles: Profiles) -> &mut Self {
        self.profiles = Some(profiles);
        self
    }

    pub fn targets(&mut self, targets: Targets) -> &mut Self {
        self.targets = Some(targets);
        self
    }

    pub fn build(&self) -> Result<Project> {
        Ok(Project {
            name:     self.name.as_ref().ok_or(Error::NoName)?.clone(),
            version:  self.version.as_ref().ok_or(Error::NoVersion)?.clone(),
            profiles: self.profiles.as_ref().ok_or(Error::NoProfiles)?.clone(),
            targets:  self.targets.as_ref().ok_or(Error::NoTargets)?.clone(),
        })
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn project() {
        assert_eq!(Project::default(), Project::builder().build());
    }
}
