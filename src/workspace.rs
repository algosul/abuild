use std::{fmt::Display, path::PathBuf};
pub mod commands;
use crate::project::Project;
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Error {
    BuilderArg(String),
}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::BuilderArg(message) => {
                write!(f, "Builder argument error: {message}")
            }
        }
    }
}
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> { None }
}
pub type Result<T> = std::result::Result<T, Error>;
#[derive(Debug, Clone, PartialEq)]
pub struct Workspace {
    name:      String,
    directory: PathBuf,
    projects:  Vec<Project>,
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct WorkspaceBuilder {
    name:      Option<String>,
    directory: Option<PathBuf>,
    projects:  Vec<Project>,
}
impl Workspace {
    pub fn try_default() -> Result<Self> { Self::builder().build() }

    pub fn builder() -> WorkspaceBuilder { WorkspaceBuilder::default() }
}
impl WorkspaceBuilder {
    pub fn build(&self) -> Result<Workspace> {
        Ok(Workspace {
            name:      self
                .name
                .as_ref()
                .ok_or(Error::BuilderArg("no name provided".to_string()))?
                .clone(),
            directory: self
                .directory
                .as_ref()
                .ok_or(Error::BuilderArg("no directory provided".to_string()))?
                .clone(),
            projects:  self.projects.clone(),
        })
    }

    pub fn name(&mut self, name: impl Into<String>) -> &mut Self {
        self.name = Some(name.into());
        self
    }

    pub fn directory(&mut self, directory: impl Into<PathBuf>) -> &mut Self {
        self.directory = Some(directory.into());
        self
    }

    pub fn project(&mut self, project: Project) -> &mut Self {
        self.projects.push(project);
        self
    }

    pub fn with_mut_projects(
        &mut self, f: impl FnOnce(&mut Vec<Project>),
    ) -> &mut Self {
        f(&mut self.projects);
        self
    }
}
#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    pub(crate) fn workspace()
    -> std::result::Result<Workspace, Box<dyn std::error::Error>> {
        Workspace::builder()
            .name("test")
            .directory(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/workspace"))
            .project(crate::project::tests::project()?)
            .build()
            .map_err(Into::into)
    }
    #[test]
    fn test_workspace() { workspace().unwrap(); }
}
