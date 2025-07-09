use std::{borrow::Cow, path::Path};

use clap::{value_parser, Parser, ValueHint};
use clap_complete::Shell;
use colored::Colorize;

use crate::command::Command;
#[derive(Debug, Clone, PartialEq, Eq, Parser)]
pub struct New {}
#[derive(Debug, Clone, PartialEq, Eq, Parser)]
pub struct Init {}
impl Command<()> for New {
    fn do_it(
        &self, _parent: &(),
    ) -> Result<Box<dyn std::any::Any>, Box<dyn std::error::Error>> {
        todo!()
    }

    fn undo_it(
        &self, _parent: &(), _cache: &mut dyn std::any::Any,
    ) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    fn redo_it(
        &self, _parent: &(), _cache: &mut dyn std::any::Any,
    ) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
}
impl Command<()> for Init {
    fn do_it(
        &self, _parent: &(),
    ) -> Result<Box<dyn std::any::Any>, Box<dyn std::error::Error>> {
        todo!()
    }

    fn undo_it(
        &self, _parent: &(), _cache: &mut dyn std::any::Any,
    ) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    fn redo_it(
        &self, _parent: &(), _cache: &mut dyn std::any::Any,
    ) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Parser)]
pub struct ShellOptions {
    /// the shell to generate the auto-completion script for
    #[clap(value_parser = value_parser!(clap_complete::Shell))]
    shell: clap_complete::Shell,
}
/// Options for the scope of the command (workspace, project, profile)
#[derive(Default, Debug, Clone, PartialEq, Eq, Parser)]
pub struct ScopeOptions {
    /// set the workspace directory
    #[clap(short, long, value_hint = ValueHint::DirPath)]
    pub workspace: Option<std::path::PathBuf>,
    /// set the project name
    #[clap(short = 'j', long, value_hint = ValueHint::DirPath)]
    pub project:   Option<String>,
    /// set the profile name
    #[clap(short, long, value_hint = ValueHint::Unknown)]
    pub profile:   Option<String>,
}
#[derive(Debug, Clone, PartialEq, Eq, Parser)]
pub enum AutoCompleteCommand {
    /// install auto-completion script
    Install {
        #[clap(flatten)]
        shell: ShellOptions,
    },
    /// reinstall auto-completion script
    Reinstall {
        #[clap(flatten)]
        shell: ShellOptions,
    },
    /// remove auto-completion script
    Remove {
        #[clap(flatten)]
        shell: ShellOptions,
    },
    /// output auto-completion script
    Output {
        #[clap(flatten)]
        shell: ShellOptions,
    },
}
impl Command<()> for AutoCompleteCommand {
    fn do_it(
        &self, _parent: &(),
    ) -> Result<Box<dyn std::any::Any>, Box<dyn std::error::Error>> {
        todo!()
    }

    fn undo_it(
        &self, _parent: &(), _cache: &mut dyn std::any::Any,
    ) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    fn redo_it(
        &self, _parent: &(), _cache: &mut dyn std::any::Any,
    ) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
}
impl ShellOptions {
    pub fn config_dir(&self) -> std::io::Result<Cow<'static, Path>> {
        match self.shell {
            Shell::Bash => {
                Ok(Cow::Borrowed(Path::new("/etc/bash_completion.d")))
            }
            Shell::Zsh => Ok(Cow::Borrowed(Path::new(
                "/usr/local/share/zsh/site-functions",
            ))),
            Shell::Fish => Ok(Cow::Borrowed(Path::new(
                "/usr/share/fish/vendor_completions.d",
            ))),
            Shell::PowerShell => Ok(Cow::Borrowed(Path::new(
                "/usr/local/share/powershell/Modules/",
            ))),
            Shell::Elvish => {
                Ok(Cow::Borrowed(Path::new("/usr/share/elvish/lib/")))
            }
            shell => panic!("unsupported shell: {shell}"),
        }
    }

    pub fn config_file_name(&self) -> Cow<'static, Path> {
        use crate::NAME;
        match self.shell {
            Shell::Bash => Cow::Borrowed(Path::new(NAME)),
            Shell::Zsh => Cow::Owned(format!("_{NAME}").into()),
            Shell::Fish => Cow::Owned(format!("{NAME}.fish").into()),
            Shell::PowerShell => Cow::Owned(format!("{NAME}.ps1").into()),
            Shell::Elvish => Cow::Owned(format!("_{NAME}.elv").into()),
            shell => panic!("unsupported shell: {shell}"),
        }
    }

    pub fn config_file_path(&self) -> std::io::Result<Cow<'static, Path>> {
        let config_dir = self.config_dir()?;
        if !config_dir.exists() {
            std::fs::create_dir_all(&config_dir)?;
        }
        Ok(Cow::Owned(config_dir.join(self.config_file_name())))
    }

    pub fn show_installed_info(&self, config_file_path: Cow<Path>) {
        match self.shell {
            Shell::PowerShell => {
                println!(
                    "{}: Please run 'Import-Module \"{}\"' in powershell",
                    "INFO".bright_white(),
                    config_file_path.display()
                );
            }
            shell @ (Shell::Bash | Shell::Zsh | Shell::Fish) => {
                println!(
                    "{}: Please reset {1}, or run 'source \"{2}\"' in {1}",
                    "INFO".bright_white(),
                    shell,
                    config_file_path.display()
                );
            }
            Shell::Elvish => {
                println!(
                    r#"{}: Please reset {1}, or run '{1} "{2}"',
    For more information, see https://github.com/zzamboni/elvish-completions."#,
                    "INFO".bright_white(),
                    Shell::Elvish,
                    config_file_path.display(),
                );
            }
            shell => panic!("unsupported shell: {shell}"),
        }
    }
}
