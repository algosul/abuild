use std::{
    any::Any,
    collections::HashMap,
    fmt::{Debug, Display, Formatter},
    sync::Arc,
};

use clap::CommandFactory;
pub mod commands;
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Error {
    DuplicateCommandName(String),
}
impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::DuplicateCommandName(name) => {
                write!(f, "Duplicate command name: {name}")
            }
        }
    }
}
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::DuplicateCommandName(_) => None,
        }
    }
}
pub type Result<T> = std::result::Result<T, Error>;
/// # Command Implementation Example
///
/// <details>
/// <summary>Impl Command Example</summary>
///
/// ```rust
/// # use std::any::Any;
/// # use std::error::Error;
/// # use clap::Parser;
/// # use abuild::command::Command;
/// # use abuild::project::Project;
/// #[derive(Debug, PartialEq, Eq, Parser)]
/// pub struct CustomCommand {}
/// impl Command<()> for CustomCommand {
///     fn do_it(&self, _: &()) -> Result<Box<dyn Any>, Box<dyn Error>> {
///         todo!()
///     }
///
///     fn undo_it(
///         &self, _: &(), cache: &mut dyn Any,
///     ) -> Result<(), Box<dyn Error>> {
///         todo!()
///     }
///
///     fn redo_it(
///         &self, _: &(), cache: &mut dyn Any,
///     ) -> Result<(), Box<dyn Error>> {
///         todo!()
///     }
/// }
/// ```
/// </details>
#[allow(private_bounds)]
pub trait Command<T>:
    Debug + Any + CommandEq<T> + ClapCommandAdapter<T>
{
    fn do_it(
        &self, parent: &T,
    ) -> std::result::Result<Box<dyn Any>, Box<dyn std::error::Error>>;
    fn undo_it(
        &self, parent: &T, cache: &mut dyn Any,
    ) -> std::result::Result<(), Box<dyn std::error::Error>>;
    fn redo_it(
        &self, parent: &T, cache: &mut dyn Any,
    ) -> std::result::Result<(), Box<dyn std::error::Error>>;
}
trait CommandEq<T> {
    fn eq(&self, other: &dyn Command<T>) -> bool;
}
pub trait ClapCommandAdapter<T> {
    fn clap_command(&self) -> clap::Command;
}
impl<T: Command<U> + CommandFactory, U> ClapCommandAdapter<U> for T {
    fn clap_command(&self) -> clap::Command {
        <Self as CommandFactory>::command()
    }
}
impl<T: Command<U> + PartialEq + Eq, U> CommandEq<U> for T {
    fn eq(&self, other: &dyn Command<U>) -> bool {
        match (other as &dyn Any).downcast_ref::<T>() {
            Some(other) => self == other,
            None => false,
        }
    }
}
#[derive(Debug, Clone)]
pub struct Commands<T> {
    commands: HashMap<String, Arc<dyn Command<T>>>,
}
#[derive(Debug, Clone)]
pub struct CommandBuilder<T> {
    commands: HashMap<String, Arc<dyn Command<T>>>,
}
impl<T> PartialEq for Commands<T> {
    fn eq(&self, other: &Self) -> bool {
        self.commands.iter().all(|(k1, v1)| match other.commands.get(k1) {
            Some(v2) => v1.eq(v2.as_ref()),
            None => false,
        })
    }
}
impl<T> Eq for Commands<T> {}
impl<T> Commands<T> {
    pub fn builder() -> CommandBuilder<T> {
        CommandBuilder { commands: HashMap::new() }
    }

    pub fn to_clap_command(&self) -> clap::Command {
        clap::Command::default().subcommands(
            self.commands
                .iter()
                .map(|(name, command)| command.clap_command().name(name)),
        )
    }
}
impl<T> CommandBuilder<T> {
    pub fn command(
        &mut self, name: impl Into<String>, command: impl Command<T>,
    ) -> Result<&mut Self> {
        let name = name.into();
        if self.commands.contains_key(&name) {
            return Err(Error::DuplicateCommandName(name.clone()));
        }
        self.commands.insert(name, Arc::new(command));
        Ok(self)
    }

    pub fn with_mut_commands(
        &mut self, f: impl Fn(&mut HashMap<String, Arc<dyn Command<T>>>),
    ) -> &mut Self {
        f(&mut self.commands);
        self
    }

    pub fn build(&self) -> Result<Commands<T>> {
        Ok(Commands { commands: self.commands.clone() })
    }
}
