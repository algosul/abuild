use std::{any::Any, error::Error, fmt::Debug};

use clap::Parser;

use crate::{
    command::{Command, Commands},
    project::Project,
};
#[derive(Debug, Clone, PartialEq, Eq, Parser)]
pub struct Build {}
#[derive(Debug, Clone, PartialEq, Eq, Parser)]
pub struct Run {}
#[derive(Debug, Clone, PartialEq, Eq, Parser)]
pub struct Test {}
impl Command<Project> for Build {
    fn do_it(
        &self, _project: &Project,
    ) -> Result<Box<dyn Any>, Box<dyn Error>> {
        todo!()
    }

    fn undo_it(
        &self, _project: &Project, _cache: &mut dyn Any,
    ) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    fn redo_it(
        &self, _project: &Project, _cache: &mut dyn Any,
    ) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}
impl Command<Project> for Run {
    fn do_it(
        &self, _project: &Project,
    ) -> Result<Box<dyn Any>, Box<dyn Error>> {
        todo!()
    }

    fn undo_it(
        &self, _project: &Project, _cache: &mut dyn Any,
    ) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    fn redo_it(
        &self, _project: &Project, _cache: &mut dyn Any,
    ) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}
impl Command<Project> for Test {
    fn do_it(
        &self, _project: &Project,
    ) -> Result<Box<dyn Any>, Box<dyn Error>> {
        todo!()
    }

    fn undo_it(
        &self, _project: &Project, _cache: &mut dyn Any,
    ) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    fn redo_it(
        &self, _project: &Project, _cache: &mut dyn Any,
    ) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}
impl Commands<Project> {
    pub fn try_default() -> crate::command::Result<Self> {
        Self::builder()
            .command("build", Build {})?
            .command("run", Run {})?
            .command("test", Test {})?
            .build()
    }
}
