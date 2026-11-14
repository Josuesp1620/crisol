use std::path::PathBuf;

use crate::cli_error::CliError;

/// What the user asked the command line to do.
pub enum CliCommand {
    /// Validate a form and list its diagnostics.
    Check {
        form: PathBuf,
    },
    /// Write the generated file and create or extend the handlers file.
    Generate {
        form: PathBuf,
        output: PathBuf,
    },
    Help,
}

impl CliCommand {
    pub const USAGE: &'static str = "Usage:\n  crisol-cli check <form.crisol>\n  crisol-cli generate <form.crisol> [--out <folder>]";

    pub fn parse(arguments: &[String]) -> Result<Self, CliError> {
        match arguments.first().map(String::as_str) {
            Some("check") => Ok(CliCommand::Check {
                form: Self::form_path(arguments)?,
            }),
            Some("generate") => {
                let form = Self::form_path(arguments)?;
                let output = Self::output_folder(arguments, &form);
                Ok(CliCommand::Generate { form, output })
            }
            Some("help" | "--help" | "-h") | None => Ok(CliCommand::Help),
            Some(other) => Err(CliError::Usage(format!(
                "Unknown command {other}.\n{}",
                Self::USAGE
            ))),
        }
    }

    fn form_path(arguments: &[String]) -> Result<PathBuf, CliError> {
        match arguments.get(1) {
            Some(path) => Ok(PathBuf::from(path)),
            None => Err(CliError::Usage(format!(
                "Missing the .crisol file.\n{}",
                Self::USAGE
            ))),
        }
    }

    fn output_folder(arguments: &[String], form: &std::path::Path) -> PathBuf {
        let mut position = 2;
        while position + 1 < arguments.len() {
            if arguments[position] == "--out" {
                return PathBuf::from(&arguments[position + 1]);
            }
            position += 1;
        }
        match form.parent() {
            Some(folder) => folder.to_path_buf(),
            None => PathBuf::from("."),
        }
    }
}
