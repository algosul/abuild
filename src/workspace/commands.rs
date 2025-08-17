use std::{any::Any, error::Error};

use clap::Parser;

use crate::{command::Command, workspace::Workspace};
#[derive(Debug, Clone, PartialEq, Eq, Parser)]
pub struct New {}
#[derive(Debug, Clone, PartialEq, Eq, Parser)]
pub struct Init {}
impl Command<Workspace> for New
{
  fn execute(&self, _parent: &Workspace)
  -> Result<Box<dyn Any>, Box<dyn Error>>
  {
    todo!()
  }

  fn undo(
    &self, _parent: &Workspace, _cache: &mut dyn Any,
  ) -> Result<(), Box<dyn Error>>
  {
    todo!()
  }

  fn redo(
    &self, _parent: &Workspace, _cache: &mut dyn Any,
  ) -> Result<(), Box<dyn Error>>
  {
    todo!()
  }
}
impl Command<Workspace> for Init
{
  fn execute(&self, _parent: &Workspace)
  -> Result<Box<dyn Any>, Box<dyn Error>>
  {
    todo!()
  }

  fn undo(
    &self, _parent: &Workspace, _cache: &mut dyn Any,
  ) -> Result<(), Box<dyn Error>>
  {
    todo!()
  }

  fn redo(
    &self, _parent: &Workspace, _cache: &mut dyn Any,
  ) -> Result<(), Box<dyn Error>>
  {
    todo!()
  }
}
