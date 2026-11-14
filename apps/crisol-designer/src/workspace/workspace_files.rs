use std::fs;
use std::path::{Path, PathBuf};

use crisol_model::document::{FormDocument, FormInfo, Node, WidgetKind};
use crisol_model::geometry::Size;
use crisol_model::identity::{Identifier, TypeName};
use crisol_model::layout::Placement;
use crisol_ui::icon::IconName;
use crisol_ui::theme::{Theme, ThemeMode};
use gpui::{Context, PathPromptOptions, Window};

use super::{DesignerWorkspace, ExplorerEntry};
use crate::canvas::PreviewWindow;
use crate::project::{FormsModuleWriter, ProjectSession};
use crate::session::{CodeExport, DocumentFiles, DocumentSession};

/// Creating, opening and saving forms. Saving also writes the form's Rust code, as
/// Windows Forms rewrites `Form1.Designer.cs` on every save.
impl DesignerWorkspace {
    const FORM_EXTENSION: &'static str = "crisol";
    const SUGGESTED_FILE: &'static str = "form.crisol";
    /// Folders the explorer never shows.
    const HIDDEN_FOLDERS: [&'static str; 3] = ["target", ".git", ".cache"];
    const EXPLORER_DEPTH: usize = 3;

    pub fn new_form(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        let session = DocumentSession::new(Self::blank_document(), None, &self.catalog);
        self.open_session(session, cx);
        self.output
            .info("New form created. Save it to write its code.");
    }

    #[allow(
        clippy::unused_self,
        reason = "every file command is a method of the workspace"
    )]
    pub fn open_form(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let receiver = cx.prompt_for_paths(PathPromptOptions {
            files: true,
            directories: false,
            multiple: false,
            prompt: Some("Open a project (.crisolproj) or a form (.crisol)".into()),
        });
        cx.spawn_in(window, async move |workspace, cx| {
            if let Ok(Ok(Some(paths))) = receiver.await
                && let Some(path) = paths.into_iter().next()
            {
                let _ = workspace.update(cx, |workspace, cx| workspace.open_any(path, cx));
            }
        })
        .detach();
    }

    /// Opens a form, and the project around it when there is one.
    pub fn open_path(&mut self, path: PathBuf, cx: &mut Context<Self>) {
        for (index, session) in self.sessions.iter().enumerate() {
            if session.path() == Some(path.as_path()) {
                self.activate(index, cx);
                return;
            }
        }
        match DocumentFiles::load(&path) {
            Ok(document) => {
                if self.project.is_none() {
                    self.project = ProjectSession::find_for(&path);
                }
                if self.project.is_none() {
                    self.recent.remember(path.clone());
                }
                self.output.info(format!("Opened {}", path.display()));
                self.open_session(
                    DocumentSession::new(document, Some(path), &self.catalog),
                    cx,
                );
            }
            Err(message) => {
                self.output.failure(message);
                cx.notify();
            }
        }
    }

    pub fn save_form(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        match self.session().path().map(Path::to_path_buf) {
            Some(path) => self.save_to(path, cx),
            None => self.save_form_as(window, cx),
        }
    }

    pub fn save_form_as(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let directory = match (
            self.session().path().and_then(Path::parent),
            self.project.as_ref(),
        ) {
            (Some(folder), _) => folder.to_path_buf(),
            (None, Some(project)) => project.root().join("src").join("forms"),
            (None, None) => std::env::current_dir().unwrap_or_default(),
        };
        let receiver = cx.prompt_for_new_path(&directory, Some(Self::SUGGESTED_FILE));
        cx.spawn_in(window, async move |workspace, cx| {
            if let Ok(Ok(Some(path))) = receiver.await {
                let _ = workspace.update(cx, |workspace, cx| workspace.save_to(path, cx));
            }
        })
        .detach();
    }

    /// Saves every open form that has a file; unsaved new forms ask for one.
    pub fn save_all(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let active = self.active;
        self.save_all_quietly(cx);
        self.active = active;
        if self.session().path().is_none() {
            self.save_form_as(window, cx);
        }
    }

    pub(super) fn save_all_quietly(&mut self, cx: &mut Context<Self>) {
        let active = self.active;
        for index in 0..self.sessions.len() {
            if let Some(path) = self.sessions[index].path().map(Path::to_path_buf)
                && self.sessions[index].has_unsaved_changes()
            {
                self.active = index;
                self.save_to(path, cx);
            }
        }
        self.active = active;
    }

    /// Writes the form's code next to it without saving the form itself.
    pub fn generate_code(&mut self, cx: &mut Context<Self>) {
        let Some(path) = self.session().path().map(Path::to_path_buf) else {
            self.output.failure("Save the form before generating code.");
            cx.notify();
            return;
        };
        self.export_code(&path);
        self.refresh_explorer();
        cx.notify();
    }

    pub fn run_preview(&mut self, cx: &mut Context<Self>) {
        let document = self.session().document().clone();
        match PreviewWindow::open(document, self.catalog.clone(), cx) {
            Ok(()) => self.output.info("Preview opened."),
            Err(message) => self.output.failure(message),
        }
        cx.notify();
    }

    #[allow(
        clippy::unused_self,
        reason = "every view command is a method of the workspace"
    )]
    pub fn toggle_theme(&mut self, cx: &mut Context<Self>) {
        let next = match Theme::of(cx).mode {
            ThemeMode::Light => ThemeMode::Dark,
            ThemeMode::Dark => ThemeMode::Light,
        };
        Theme::set_mode(next, cx);
    }

    /// The project folder as a tree, or the folder of a form outside a project.
    pub(super) fn refresh_explorer(&mut self) {
        self.explorer.clear();
        let root = match (
            self.project.as_ref(),
            self.session().path().and_then(Path::parent),
        ) {
            (Some(project), _) => project.root(),
            (None, Some(folder)) => folder.to_path_buf(),
            (None, None) => return,
        };
        let label = root
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_default();
        self.explorer.push(ExplorerEntry {
            path: root.clone(),
            label,
            icon: IconName::Box,
            depth: 0,
            generated: false,
            is_form: false,
        });
        self.collect_entries(&root, 1);
    }

    fn collect_entries(&mut self, folder: &Path, depth: usize) {
        if depth > Self::EXPLORER_DEPTH {
            return;
        }
        let Ok(reader) = fs::read_dir(folder) else {
            return;
        };
        let mut folders = Vec::new();
        let mut files = Vec::new();
        for entry in reader.flatten() {
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().into_owned();
            if path.is_dir() {
                if !Self::HIDDEN_FOLDERS.contains(&name.as_str()) {
                    folders.push(path);
                }
            } else if let Some(file) = Self::explorer_entry_for(path, depth) {
                files.push(file);
            }
        }
        folders.sort();
        files.sort_by(|first, second| first.label.cmp(&second.label));
        for path in folders {
            let label = path
                .file_name()
                .map(|name| name.to_string_lossy().into_owned())
                .unwrap_or_default();
            self.explorer.push(ExplorerEntry {
                path: path.clone(),
                label,
                icon: IconName::Folder,
                depth,
                generated: false,
                is_form: false,
            });
            self.collect_entries(&path, depth + 1);
        }
        self.explorer.extend(files);
    }

    fn explorer_entry_for(path: PathBuf, depth: usize) -> Option<ExplorerEntry> {
        let label = path.file_name()?.to_string_lossy().into_owned();
        let extension = path
            .extension()
            .map(|extension| extension.to_string_lossy().into_owned())
            .unwrap_or_default();
        let icon = match extension.as_str() {
            "crisol" => IconName::Form,
            "rs" => IconName::Rust,
            "crisolproj" => IconName::Box,
            "toml" => IconName::File,
            _ => return None,
        };
        Some(ExplorerEntry {
            generated: label.ends_with(".generated.rs"),
            is_form: extension == Self::FORM_EXTENSION,
            path,
            label,
            icon,
            depth,
        })
    }

    fn save_to(&mut self, mut path: PathBuf, cx: &mut Context<Self>) {
        if path
            .extension()
            .is_none_or(|extension| extension != Self::FORM_EXTENSION)
        {
            path.set_extension(Self::FORM_EXTENSION);
        }
        let shown = path.display().to_string();
        match self.session_mut().save_to(path.clone()) {
            Ok(()) => {
                self.output.success(format!("Saved {shown}"));
                self.export_code(&path);
                if let Some(project) = self.project.as_mut()
                    && path.starts_with(project.root())
                    && let Err(message) = project.add_form(&path)
                {
                    self.output.failure(message);
                }
                if self.project.is_none() {
                    self.recent.remember(path);
                }
            }
            Err(message) => self.output.failure(message),
        }
        self.refresh_explorer();
        cx.notify();
    }

    /// Writes the generated file, extends the handlers file and keeps `mod.rs` current.
    fn export_code(&mut self, form: &Path) {
        match CodeExport::new(&self.catalog).export(self.session().document(), form) {
            Ok(files) => self.output.info(format!("Wrote {}", files.join(" and "))),
            Err(message) => self
                .output
                .failure(format!("The code was not written: {message}")),
        }
        if let Some(folder) = form.parent()
            && folder.join("mod.rs").exists()
            && let Err(message) = FormsModuleWriter::write(folder)
        {
            self.output.failure(message);
        }
        self.reload_code_views();
    }

    fn blank_document() -> FormDocument {
        let form = FormInfo {
            name: TypeName::parse("NewForm").expect("a valid type name"),
            title: "New form".to_owned(),
            size: Size::new(520, 372),
            state: None,
        };
        let root = Node::new(
            WidgetKind::new("Form"),
            Identifier::parse("frm_main").expect("a valid name"),
            Placement::flow(),
        );
        FormDocument::new(form, root)
    }
}
