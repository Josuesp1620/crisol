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
use crate::session::{CodeExport, DocumentFiles, DocumentSession};

/// Creating, opening, saving and exporting forms.
impl DesignerWorkspace {
    const FORM_EXTENSION: &'static str = "crisol";
    const SUGGESTED_FILE: &'static str = "form.crisol";

    pub fn new_form(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        let session = DocumentSession::new(Self::blank_document(), None, &self.catalog);
        self.sessions.push(session);
        self.activate(self.sessions.len() - 1, cx);
        self.output.info("New form created.");
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
            prompt: Some("Open form".into()),
        });
        cx.spawn_in(window, async move |workspace, cx| {
            if let Ok(Ok(Some(paths))) = receiver.await
                && let Some(path) = paths.into_iter().next()
            {
                let _ = workspace.update(cx, |workspace, cx| workspace.open_path(path, cx));
            }
        })
        .detach();
    }

    pub fn open_path(&mut self, path: PathBuf, cx: &mut Context<Self>) {
        for (index, session) in self.sessions.iter().enumerate() {
            if session.path() == Some(path.as_path()) {
                self.activate(index, cx);
                return;
            }
        }
        match DocumentFiles::load(&path) {
            Ok(document) => {
                self.output.info(format!("Opened {}", path.display()));
                self.sessions
                    .push(DocumentSession::new(document, Some(path), &self.catalog));
                self.activate(self.sessions.len() - 1, cx);
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
        let directory = match self.session().path().and_then(Path::parent) {
            Some(folder) => folder.to_path_buf(),
            None => std::env::current_dir().unwrap_or_default(),
        };
        let receiver = cx.prompt_for_new_path(&directory, Some(Self::SUGGESTED_FILE));
        cx.spawn_in(window, async move |workspace, cx| {
            if let Ok(Ok(Some(path))) = receiver.await {
                let _ = workspace.update(cx, |workspace, cx| workspace.save_to(path, cx));
            }
        })
        .detach();
    }

    pub fn generate_code(&mut self, cx: &mut Context<Self>) {
        let Some(folder) = self
            .session()
            .path()
            .and_then(Path::parent)
            .map(Path::to_path_buf)
        else {
            self.output.failure("Save the form before generating code.");
            cx.notify();
            return;
        };
        match CodeExport::new(&self.catalog).export(self.session().document(), &folder) {
            Ok(files) => self
                .output
                .success(format!("Generated {}", files.join(" and "))),
            Err(message) => self.output.failure(message),
        }
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

    pub(super) fn refresh_explorer(&mut self) {
        self.explorer.clear();
        let Some(folder) = self
            .session()
            .path()
            .and_then(Path::parent)
            .map(Path::to_path_buf)
        else {
            return;
        };
        let folder_name = folder
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_default();
        self.explorer.push(ExplorerEntry {
            path: folder.clone(),
            label: folder_name,
            icon: IconName::Folder,
            depth: 0,
            generated: false,
            is_form: false,
        });
        let Ok(reader) = fs::read_dir(&folder) else {
            return;
        };
        let mut files = Vec::new();
        for entry in reader.flatten() {
            let path = entry.path();
            if let Some(file) = Self::explorer_entry_for(path) {
                files.push(file);
            }
        }
        files.sort_by(|first, second| first.label.cmp(&second.label));
        self.explorer.extend(files);
    }

    fn explorer_entry_for(path: PathBuf) -> Option<ExplorerEntry> {
        let label = path.file_name()?.to_string_lossy().into_owned();
        let is_form = path
            .extension()
            .is_some_and(|extension| extension == Self::FORM_EXTENSION);
        let generated = label.ends_with(".generated.rs");
        let is_rust = path.extension().is_some_and(|extension| extension == "rs");
        if !is_form && !is_rust && label != "Cargo.toml" {
            return None;
        }
        let icon = if is_form {
            IconName::Form
        } else if is_rust {
            IconName::Rust
        } else {
            IconName::File
        };
        Some(ExplorerEntry {
            path,
            label,
            icon,
            depth: 1,
            generated,
            is_form,
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
        match self.session_mut().save_to(path) {
            Ok(()) => self.output.success(format!("Saved {shown}")),
            Err(message) => self.output.failure(message),
        }
        self.refresh_explorer();
        cx.notify();
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
