use std::process::ExitCode;

use crate::check_command::CheckCommand;
use crate::cli_command::CliCommand;
use crate::cli_error::CliError;
use crate::generate_command::GenerateCommand;

/// Parses the arguments, runs the command and turns the outcome into an exit code.
pub struct CliApplication {
    arguments: Vec<String>,
}

impl CliApplication {
    pub fn new(arguments: Vec<String>) -> Self {
        Self { arguments }
    }

    pub fn run(&self) -> ExitCode {
        match self.execute() {
            Ok(true) => ExitCode::SUCCESS,
            Ok(false) => ExitCode::FAILURE,
            Err(error) => {
                eprintln!("{error}");
                ExitCode::FAILURE
            }
        }
    }

    fn execute(&self) -> Result<bool, CliError> {
        match CliCommand::parse(&self.arguments)? {
            CliCommand::Check { form } => CheckCommand::run(&form),
            CliCommand::Generate { form, output } => {
                GenerateCommand::run(&form, &output).map(|()| true)
            }
            CliCommand::Help => {
                println!("{}", CliCommand::USAGE);
                Ok(true)
            }
        }
    }
}
