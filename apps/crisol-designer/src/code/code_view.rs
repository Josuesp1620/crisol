use std::fs;
use std::path::{Path, PathBuf};

/// A Rust file open in a code tab.
#[derive(Debug, Clone)]
pub struct CodeView {
    path: PathBuf,
    lines: Vec<String>,
    /// Generated files are rewritten by Crisol, so they are only read.
    generated: bool,
    /// A line to bring into view and mark, like the method a double-click jumped to.
    marked_line: Option<usize>,
}

impl CodeView {
    pub fn open(path: &Path) -> Result<Self, String> {
        let text = fs::read_to_string(path)
            .map_err(|error| format!("Cannot read {}: {error}", path.display()))?;
        let mut lines = Vec::new();
        for line in text.lines() {
            lines.push(line.to_owned());
        }
        Ok(Self {
            path: path.to_path_buf(),
            lines,
            generated: path.to_string_lossy().ends_with(".generated.rs"),
            marked_line: None,
        })
    }

    /// Reads the file again, after Crisol regenerated or extended it.
    pub fn reload(&mut self) {
        if let Ok(fresh) = Self::open(&self.path) {
            let marked = self.marked_line;
            *self = fresh;
            self.marked_line = marked;
        }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn file_name(&self) -> String {
        self.path
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_default()
    }

    pub fn lines(&self) -> &[String] {
        &self.lines
    }

    pub fn is_generated(&self) -> bool {
        self.generated
    }

    pub fn marked_line(&self) -> Option<usize> {
        self.marked_line
    }

    /// Marks the line that declares `fn <method>(`, if there is one.
    pub fn mark_method(&mut self, method: &str) {
        let declaration = format!("fn {method}(");
        self.marked_line = self
            .lines
            .iter()
            .position(|line| line.contains(&declaration));
    }
}
