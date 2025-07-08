use std::fmt::Display;

use crate::project::Project;
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Error {
    NoName,
}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::NoName => write!(f, "No name provided"),
        }
    }
}
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> { None }
}
pub type Result<T> = std::result::Result<T, Error>;
#[derive(Debug, Clone, PartialEq)]
pub struct Workspace {
    name:     String,
    projects: Vec<Project>,
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct WorkspaceBuilder {
    name:     Option<String>,
    projects: Vec<Project>,
}
impl Workspace {
    pub fn default() -> Result<Self> { Self::builder().build() }

    pub fn builder() -> WorkspaceBuilder { WorkspaceBuilder::default() }
}
impl WorkspaceBuilder {
    pub fn build(&self) -> Result<Workspace> {
        Ok(Workspace {
            name:     self.name.as_ref().ok_or(Error::NoName)?.clone(),
            projects: self.projects.clone(),
        })
    }

    pub fn name(&mut self, name: &str) -> &mut Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn project(&mut self, project: Project) -> &mut Self {
        self.projects.push(project);
        self
    }

    pub fn projects(&mut self, mut projects: Vec<Project>) -> &mut Self {
        self.projects.append(&mut projects);
        self
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn workspace() {
        assert_eq!(
            Workspace::default(),
            Workspace::builder().build()
        );
    }
}
