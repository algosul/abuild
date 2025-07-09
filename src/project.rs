use std::{
    fmt::{Debug, Display},
    path::PathBuf,
};

use crate::{
    command::Commands,
    feature::Features,
    profile::Profiles,
    target::Targets,
};
pub mod commands;
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Error {
    BuilderArg(String),
    ProfileError(crate::profile::Error),
    TargetError(crate::target::Error),
    FeatureError(crate::feature::Error),
    CommandError(crate::command::Error),
}
impl From<crate::profile::Error> for Error {
    fn from(value: crate::profile::Error) -> Self { Error::ProfileError(value) }
}
impl From<crate::target::Error> for Error {
    fn from(value: crate::target::Error) -> Self { Error::TargetError(value) }
}
impl From<crate::feature::Error> for Error {
    fn from(value: crate::feature::Error) -> Self { Error::FeatureError(value) }
}
impl From<crate::command::Error> for Error {
    fn from(value: crate::command::Error) -> Self { Error::CommandError(value) }
}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::BuilderArg(message) => {
                write!(f, "Builder argument error: {message}")
            }
            Error::ProfileError(error) => {
                write!(f, "Profile error: {error}")
            }
            Error::TargetError(error) => {
                write!(f, "Target error: {error}")
            }
            Error::FeatureError(error) => {
                write!(f, "Feature error: {error}")
            }
            Error::CommandError(error) => {
                write!(f, "Command error: {error}")
            }
        }
    }
}
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::BuilderArg(_) => None,
            Error::ProfileError(error) => Some(error),
            Error::TargetError(error) => Some(error),
            Error::FeatureError(error) => Some(error),
            Error::CommandError(error) => Some(error),
        }
    }
}
pub type Result<T> = std::result::Result<T, Error>;
#[derive(Debug, Clone, PartialEq)]
pub struct Project {
    name:               String,
    relative_directory: PathBuf,
    version:            String,
    profiles:           Profiles,
    targets:            Targets,
    features:           Features,
    commands:           Commands<Project>,
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ProjectBuilder {
    name:               Option<String>,
    relative_directory: Option<PathBuf>,
    version:            Option<String>,
    profiles:           Option<Profiles>,
    targets:            Option<Targets>,
    features:           Option<Features>,
    commands:           Option<Commands<Project>>,
}
impl Project {
    pub fn try_default() -> Result<Self> { Self::builder().build() }

    pub fn builder() -> ProjectBuilder { ProjectBuilder::default() }
}
impl ProjectBuilder {
    pub fn name(&mut self, name: impl Into<String>) -> &mut Self {
        self.name = Some(name.into());
        self
    }

    pub fn relative_directory(
        &mut self, relative_directory: impl Into<PathBuf>,
    ) -> &mut Self {
        self.relative_directory = Some(relative_directory.into());
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

    pub fn commands(&mut self, commands: Commands<Project>) -> &mut Self {
        self.commands = Some(commands);
        self
    }

    pub fn features(&mut self, feature: Features) -> &mut Self {
        self.features = Some(feature);
        self
    }

    pub fn build(&self) -> Result<Project> {
        Ok(Project {
            name:               self
                .name
                .as_ref()
                .ok_or(Error::BuilderArg("No name provided".to_string()))?
                .clone(),
            relative_directory: self
                .relative_directory
                .as_ref()
                .ok_or(Error::BuilderArg(
                    "No relative_directory provided".to_string(),
                ))?
                .clone(),
            version:            self
                .version
                .as_ref()
                .ok_or(Error::BuilderArg("No version provided".to_string()))?
                .clone(),
            profiles:           self
                .profiles
                .as_ref()
                .ok_or(Error::BuilderArg("No profiles provided".to_string()))?
                .clone(),
            targets:            self
                .targets
                .as_ref()
                .ok_or(Error::BuilderArg("No targets provided".to_string()))?
                .clone(),
            features:           self
                .features
                .as_ref()
                .ok_or(Error::BuilderArg("No features provided".to_string()))?
                .clone(),
            commands:           self
                .commands
                .as_ref()
                .ok_or(Error::BuilderArg("No commands provided".to_string()))?
                .clone(),
        })
    }
}
#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    pub(crate) fn project() -> Result<Project> {
        Project::builder()
            .name("test")
            .relative_directory("project")
            .version("0.1.0")
            .profiles(Profiles::try_default()?)
            .targets(Targets::try_default()?)
            .features(Features::try_default()?)
            .commands(Commands::<Project>::try_default()?)
            .build()
    }
    #[test]
    fn test_project() { project().unwrap(); }
}
