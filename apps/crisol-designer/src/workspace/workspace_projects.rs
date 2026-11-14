use std::path::PathBuf;

use crisol_ui::components::TextBox;
use crisol_ui::theme::ThemeMode;
use gpui::{AppContext, Context, PathPromptOptions, Window};

use super::{DesignerWorkspace, NewProjectState};
use crate::designer_application::DesignerApplication;
use crate::designer_texts::DesignerTexts;
use crate::project::{
    CargoRunner, FormsModuleWriter, ProjectScaffolder, ProjectSession, ProjectTemplate,
};
use crate::session::{CodeExport, DocumentFiles, DocumentSession};
use crisol_model::project::ProjectSerializer;

/// The New project dialog, recent projects and the sample form.
impl DesignerWorkspace {
    pub fn open_new_project(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        self.open_new_project_without_window(cx);
    }

    /// Opens the New project dialog; the window is not needed to build it.
    pub fn open_new_project_without_window(&mut self, cx: &mut Context<Self>) {
        let location = Self::default_location().to_string_lossy().into_owned();
        let name = cx.new(|cx| TextBox::new(cx).text(DesignerTexts::DEFAULT_PROJECT_NAME));
        let location = cx.new(|cx| TextBox::new(cx).text(&location));
        self.open_menu = None;
        self.new_project = Some(NewProjectState {
            template: ProjectTemplate::DesktopApplication,
            name,
            location,
            theme: ThemeMode::Light,
            error: None,
        });
        cx.notify();
    }

    pub fn close_new_project(&mut self, cx: &mut Context<Self>) {
        self.new_project = None;
        cx.notify();
    }

    pub fn choose_template(&mut self, template: ProjectTemplate, cx: &mut Context<Self>) {
        if let Some(state) = self.new_project.as_mut() {
            state.template = template;
            cx.notify();
        }
    }

    pub fn choose_project_theme(&mut self, theme: ThemeMode, cx: &mut Context<Self>) {
        if let Some(state) = self.new_project.as_mut() {
            state.theme = theme;
            cx.notify();
        }
    }

    pub fn browse_location(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let receiver = cx.prompt_for_paths(PathPromptOptions {
            files: false,
            directories: true,
            multiple: false,
            prompt: Some("Choose the folder for the project".into()),
        });
        cx.spawn_in(window, async move |workspace, cx| {
            if let Ok(Ok(Some(paths))) = receiver.await
                && let Some(folder) = paths.into_iter().next()
            {
                let _ = workspace.update(cx, |workspace, cx| {
                    if let Some(state) = workspace.new_project.as_ref() {
                        let text = folder.to_string_lossy().into_owned();
                        state
                            .location
                            .update(cx, |field, cx| field.set_value(&text, cx));
                    }
                });
            }
        })
        .detach();
    }

    pub fn create_project(&mut self, cx: &mut Context<Self>) {
        let Some(state) = self.new_project.as_ref() else {
            return;
        };
        let name = state.name.read(cx).value().trim().to_owned();
        let location = PathBuf::from(state.location.read(cx).value().trim());
        let template = state.template;
        let theme = state.theme;
        match ProjectScaffolder::new(&self.catalog).create(&name, &location, template, theme) {
            Ok(form) => {
                self.new_project = None;
                self.output
                    .success(format!("Created project {name} in {}", location.display()));
                self.open_path(form, cx);
            }
            Err(message) => {
                if let Some(state) = self.new_project.as_mut() {
                    state.error = Some(message);
                }
                cx.notify();
            }
        }
    }

    /// Opens a `.crisolproj` project or a `.crisol` form.
    pub fn open_any(&mut self, path: PathBuf, cx: &mut Context<Self>) {
        if path
            .extension()
            .is_some_and(|extension| extension == ProjectSerializer::EXTENSION)
        {
            self.open_project(path, cx);
        } else {
            self.open_path(path, cx);
        }
    }

    /// Opens a project and its startup form.
    pub fn open_project(&mut self, manifest: PathBuf, cx: &mut Context<Self>) {
        match ProjectSession::load(&manifest) {
            Ok(project) => {
                let startup = project.startup_form();
                self.output
                    .info(format!("Opened project {}", project.name()));
                self.project = Some(project);
                self.recent.remember(manifest);
                if let Some(form) = startup {
                    self.open_path(form, cx);
                } else {
                    self.refresh_explorer();
                    cx.notify();
                }
            }
            Err(message) => {
                self.output.failure(message);
                cx.notify();
            }
        }
    }

    /// Adds `FormN` to the open project, with its code, and opens it.
    pub fn add_form(&mut self, cx: &mut Context<Self>) {
        let Some(project) = self.project.as_ref() else {
            self.output
                .failure("Open or create a project to add forms to it.");
            self.show_bottom_tab(super::BottomTab::Output, cx);
            return;
        };
        let forms = project.root().join("src").join("forms");
        let mut number = project.form_paths().len() + 1;
        let mut path = forms.join(format!("form{number}.crisol"));
        while path.exists() {
            number += 1;
            path = forms.join(format!("form{number}.crisol"));
        }
        let mut document =
            ProjectTemplate::Empty.main_form(&format!("Form {number}"), &self.catalog);
        if let Ok(name) = crisol_model::identity::TypeName::parse(&format!("Form{number}")) {
            document.form_mut().name = name;
        }
        let saved = DocumentFiles::save(&document, &path)
            .and_then(|()| {
                CodeExport::new(&self.catalog)
                    .export(&document, &path)
                    .map(|_| ())
            })
            .and_then(|()| FormsModuleWriter::write(&forms));
        let added = match (saved, self.project.as_mut()) {
            (Ok(()), Some(project)) => project.add_form(&path),
            (Err(message), _) => Err(message),
            (Ok(()), None) => Ok(()),
        };
        match added {
            Ok(()) => {
                self.output.success(format!("Added {}", path.display()));
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

    /// Saves every form, then runs `cargo build` in the background.
    pub fn build_project(&mut self, cx: &mut Context<Self>) {
        let Some(root) = self.project.as_ref().map(ProjectSession::root) else {
            self.output.failure("Open a project to build it.");
            self.show_bottom_tab(super::BottomTab::Output, cx);
            return;
        };
        if self.building {
            return;
        }
        self.save_all_quietly(cx);
        self.building = true;
        self.output.info("Building with cargo…");
        self.show_bottom_tab(super::BottomTab::Output, cx);
        let build = cx
            .background_executor()
            .spawn(async move { CargoRunner::new(&root).build() });
        cx.spawn(async move |workspace, cx| {
            let result = build.await;
            let _ = workspace.update(cx, |workspace, cx| {
                workspace.building = false;
                match result {
                    Ok((success, text)) => {
                        for line in text.lines() {
                            workspace.output.info(line.to_owned());
                        }
                        if success {
                            workspace.output.success("Build succeeded.");
                        } else {
                            workspace
                                .output
                                .failure("Build failed. See the lines above.");
                        }
                    }
                    Err(message) => workspace.output.failure(message),
                }
                cx.notify();
            });
        })
        .detach();
    }

    /// Saves every form and starts the application with `cargo run`.
    pub fn run_project(&mut self, cx: &mut Context<Self>) {
        let Some(root) = self.project.as_ref().map(ProjectSession::root) else {
            self.output
                .failure("Open a project to run it. F5 previews the form without building.");
            self.show_bottom_tab(super::BottomTab::Output, cx);
            return;
        };
        self.save_all_quietly(cx);
        match CargoRunner::new(&root).run() {
            Ok(()) => self.output.info(
                "Starting the application with cargo run. The first build takes a few minutes.",
            ),
            Err(message) => self.output.failure(message),
        }
        self.show_bottom_tab(super::BottomTab::Output, cx);
    }

    pub fn open_sample(&mut self, cx: &mut Context<Self>) {
        self.open_session(DesignerApplication::sample_session(&self.catalog), cx);
    }

    /// The user's documents folder, or the home folder, or the current folder.
    fn default_location() -> PathBuf {
        let home = std::env::var_os("USERPROFILE").or_else(|| std::env::var_os("HOME"));
        match home {
            Some(home) => {
                let documents = PathBuf::from(&home).join("Documents");
                if documents.exists() {
                    documents
                } else {
                    PathBuf::from(home)
                }
            }
            None => std::env::current_dir().unwrap_or_default(),
        }
    }
}
