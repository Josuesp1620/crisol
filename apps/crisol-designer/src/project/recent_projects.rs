use std::fs;
use std::path::PathBuf;

/// Forms opened lately, newest first, kept in the user's configuration folder.
#[derive(Debug, Default)]
pub struct RecentProjects {
    forms: Vec<PathBuf>,
}

impl RecentProjects {
    const CAPACITY: usize = 8;
    const FILE_NAME: &'static str = "recent.txt";

    pub fn load() -> Self {
        let mut forms = Vec::new();
        if let Some(file) = Self::file()
            && let Ok(text) = fs::read_to_string(file)
        {
            for line in text.lines() {
                let path = PathBuf::from(line);
                if path.exists() {
                    forms.push(path);
                }
            }
        }
        Self { forms }
    }

    pub fn forms(&self) -> &[PathBuf] {
        &self.forms
    }

    /// Moves `form` to the top and saves the list.
    pub fn remember(&mut self, form: PathBuf) {
        self.forms.retain(|known| *known != form);
        self.forms.insert(0, form);
        self.forms.truncate(Self::CAPACITY);
        self.save();
    }

    fn save(&self) {
        let Some(file) = Self::file() else {
            return;
        };
        if let Some(folder) = file.parent() {
            let _ = fs::create_dir_all(folder);
        }
        let mut text = String::new();
        for form in &self.forms {
            text.push_str(&form.to_string_lossy());
            text.push('\n');
        }
        let _ = fs::write(file, text);
    }

    /// `%APPDATA%\Crisol` on Windows, `$XDG_CONFIG_HOME/crisol` or `~/.config/crisol` elsewhere.
    fn file() -> Option<PathBuf> {
        let folder = if cfg!(target_os = "windows") {
            PathBuf::from(std::env::var_os("APPDATA")?).join("Crisol")
        } else if let Some(config) = std::env::var_os("XDG_CONFIG_HOME") {
            PathBuf::from(config).join("crisol")
        } else {
            PathBuf::from(std::env::var_os("HOME")?)
                .join(".config")
                .join("crisol")
        };
        Some(folder.join(Self::FILE_NAME))
    }
}
