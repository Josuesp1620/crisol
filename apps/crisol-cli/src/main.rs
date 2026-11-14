mod check_command;
mod cli_application;
mod cli_command;
mod cli_error;
mod document_loader;
mod generate_command;

use std::process::ExitCode;

use cli_application::CliApplication;

fn main() -> ExitCode {
    CliApplication::new(std::env::args().skip(1).collect()).run()
}
