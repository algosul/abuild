use std::{error::Error, path::PathBuf};

use crate::{module::Module, project::Project};
#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct ProjectSourceDirectory {
    src: PathBuf,
}
#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct ProjectBuild;
impl Module<Project> for ProjectSourceDirectory {
    fn do_it(
        &mut self, parent: &mut Project,
        dependencies: (&Self::D0, &Self::D1, &Self::D2),
    ) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    fn redo_it(
        &mut self, parent: &mut Project,
        dependencies: (&Self::D0, &Self::D1, &Self::D2),
    ) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    fn undo_it(
        &mut self, parent: &mut Project,
        dependencies: (&Self::D0, &Self::D1, &Self::D2),
    ) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}
impl Module<Project> for ProjectBuild {
    type D0 = ProjectSourceDirectory;

    fn do_it(
        &mut self, parent: &mut Project,
        dependencies: (&Self::D0, &Self::D1, &Self::D2),
    ) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    fn redo_it(
        &mut self, parent: &mut Project,
        dependencies: (&Self::D0, &Self::D1, &Self::D2),
    ) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    fn undo_it(
        &mut self, parent: &mut Project,
        dependencies: (&Self::D0, &Self::D1, &Self::D2),
    ) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}
