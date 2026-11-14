use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// Builds and runs a project with the Rust toolchain installed on this computer.
pub struct CargoRunner {
    root: PathBuf,
}

impl CargoRunner {
    /// Windows: start cargo without opening a console window.
    #[cfg(target_os = "windows")]
    const NO_WINDOW: u32 = 0x0800_0000;

    pub fn new(root: &Path) -> Self {
        Self {
            root: root.to_path_buf(),
        }
    }

    /// Runs `cargo build` and returns its output, or why it could not run.
    pub fn build(&self) -> Result<(bool, String), String> {
        let output = self
            .command(&["build"])
            .output()
            .map_err(|error| Self::explain(&error))?;
        let mut text = String::from_utf8_lossy(&output.stdout).into_owned();
        text.push_str(&String::from_utf8_lossy(&output.stderr));
        Ok((output.status.success(), text))
    }

    /// Starts `cargo run` and returns at once; the application opens its own window.
    pub fn run(&self) -> Result<(), String> {
        self.command(&["run"])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .map(|_| ())
            .map_err(|error| Self::explain(&error))
    }

    fn command(&self, arguments: &[&str]) -> Command {
        let mut command = Command::new("cargo");
        command.args(arguments).current_dir(&self.root);
        #[cfg(target_os = "windows")]
        {
            use std::os::windows::process::CommandExt;
            command.creation_flags(Self::NO_WINDOW);
        }
        command
    }

    fn explain(error: &std::io::Error) -> String {
        format!(
            "Could not start cargo ({error}). Install Rust from https://rustup.rs to build and run projects."
        )
    }
}
