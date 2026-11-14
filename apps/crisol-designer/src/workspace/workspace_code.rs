use std::path::{Path, PathBuf};

use gpui::Context;

use super::DesignerWorkspace;
use crate::code::CodeView;
use crate::session::CodeExport;

/// Code tabs: View Code (F7) on a form, jumping to an event's method, and opening files.
impl DesignerWorkspace {
    /// Opens the handlers file of the active form, marking `method` when given.
    pub fn view_code(&mut self, method: Option<&str>, cx: &mut Context<Self>) {
        let Some(form) = self.session().path().map(Path::to_path_buf) else {
            self.output
                .failure("Save the form first: its code lives next to the .crisol file.");
            self.show_bottom_tab(super::BottomTab::Output, cx);
            return;
        };
        let handlers = Self::handlers_path(&form);
        if !handlers.exists() {
            self.generate_code(cx);
        }
        self.open_code(&handlers, method, cx);
    }

    pub fn view_designer(&mut self, cx: &mut Context<Self>) {
        self.active_code = None;
        cx.notify();
    }

    pub fn open_code(&mut self, path: &Path, method: Option<&str>, cx: &mut Context<Self>) {
        let existing = self.code_views.iter().position(|view| view.path() == path);
        let index = match existing {
            Some(index) => {
                self.code_views[index].reload();
                index
            }
            None => match CodeView::open(path) {
                Ok(view) => {
                    self.code_views.push(view);
                    self.code_views.len() - 1
                }
                Err(message) => {
                    self.output.failure(message);
                    cx.notify();
                    return;
                }
            },
        };
        if let Some(method) = method {
            self.code_views[index].mark_method(method);
        }
        self.active_code = Some(index);
        self.show_start = false;
        cx.notify();
    }

    pub fn activate_code(&mut self, index: usize, cx: &mut Context<Self>) {
        if index < self.code_views.len() {
            self.code_views[index].reload();
            self.active_code = Some(index);
            cx.notify();
        }
    }

    pub fn close_code(&mut self, index: usize, cx: &mut Context<Self>) {
        if index < self.code_views.len() {
            self.code_views.remove(index);
            self.active_code = None;
            cx.notify();
        }
    }

    /// Opens the file in the program the system uses for `.rs` files.
    pub fn open_externally(&mut self, path: &Path, cx: &mut Context<Self>) {
        cx.open_with_system(path);
        self.output
            .info(format!("Opened {} in your editor.", path.display()));
        cx.notify();
    }

    /// Reloads code tabs after Crisol wrote files.
    pub(super) fn reload_code_views(&mut self) {
        for view in &mut self.code_views {
            view.reload();
        }
    }

    fn handlers_path(form: &Path) -> PathBuf {
        let folder = form.parent().unwrap_or_else(|| Path::new("."));
        folder.join(format!("{}_events.rs", CodeExport::module_of(form)))
    }
}
