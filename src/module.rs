use std::{any::Any, error::Error};
pub mod modules;
pub trait Module<Parent> {
    fn do_it(&mut self, parent: Parent) -> Result<(), Box<dyn Error>>;
    fn redo_it(&mut self, parent: Parent) -> Result<(), Box<dyn Error>>;
    fn undo_it(&mut self, parent: Parent) -> Result<(), Box<dyn Error>>;
    fn get_children() -> impl Iterator<Item = &dyn Module<&Self>>;
}
#[derive(Debug)]
pub struct Modules<Parent> {
    modules: Vec<Box<dyn Module<Parent>>>,
}

