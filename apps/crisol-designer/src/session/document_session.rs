use std::path::{Path, PathBuf};

use crisol_catalog::WidgetCatalog;
use crisol_catalog::validation::{CatalogValidator, Diagnostic};
use crisol_commands::Command;
use crisol_commands::history::CommandHistory;
use crisol_engine::selection::SelectionSet;
use crisol_model::document::FormDocument;
use crisol_render::LayoutRecorder;

use super::DocumentFiles;
use crate::designer_texts::DesignerTexts;

/// One open form: its document, undo history, selection and where it was drawn.
pub struct DocumentSession {
    document: FormDocument,
    history: CommandHistory,
    selection: SelectionSet,
    path: Option<PathBuf>,
    recorder: LayoutRecorder,
    diagnostics: Vec<Diagnostic>,
}

impl DocumentSession {
    pub fn new(document: FormDocument, path: Option<PathBuf>, catalog: &WidgetCatalog) -> Self {
        let mut selection = SelectionSet::default();
        selection.select_only(document.root_id());
        let mut session = Self {
            document,
            history: CommandHistory::new(),
            selection,
            path,
            recorder: LayoutRecorder::default(),
            diagnostics: Vec::new(),
        };
        session.validate(catalog);
        session
    }

    pub fn document(&self) -> &FormDocument {
        &self.document
    }

    pub fn history(&self) -> &CommandHistory {
        &self.history
    }

    pub fn selection(&self) -> &SelectionSet {
        &self.selection
    }

    pub fn selection_mut(&mut self) -> &mut SelectionSet {
        &mut self.selection
    }

    pub fn recorder(&self) -> &LayoutRecorder {
        &self.recorder
    }

    pub fn diagnostics(&self) -> &[Diagnostic] {
        &self.diagnostics
    }

    pub fn path(&self) -> Option<&Path> {
        self.path.as_deref()
    }

    pub fn file_name(&self) -> String {
        match self.path.as_ref().and_then(|path| path.file_name()) {
            Some(name) => name.to_string_lossy().into_owned(),
            None => DesignerTexts::UNSAVED_DOCUMENT.to_owned(),
        }
    }

    pub fn has_unsaved_changes(&self) -> bool {
        self.history.has_unsaved_changes() || self.path.is_none()
    }

    pub fn execute(
        &mut self,
        command: &dyn Command,
        catalog: &WidgetCatalog,
    ) -> Result<String, String> {
        self.history
            .execute(command, &mut self.document)
            .map_err(|error| error.to_string())?;
        self.after_change(catalog);
        Ok(command.label())
    }

    pub fn undo(&mut self, catalog: &WidgetCatalog) -> Result<String, String> {
        let label = self
            .history
            .undo(&mut self.document)
            .map_err(|error| error.to_string())?;
        self.after_change(catalog);
        Ok(label)
    }

    pub fn redo(&mut self, catalog: &WidgetCatalog) -> Result<String, String> {
        let label = self
            .history
            .redo(&mut self.document)
            .map_err(|error| error.to_string())?;
        self.after_change(catalog);
        Ok(label)
    }

    pub fn save_to(&mut self, path: PathBuf) -> Result<(), String> {
        DocumentFiles::save(&self.document, &path)?;
        self.path = Some(path);
        self.history.mark_saved();
        Ok(())
    }

    fn after_change(&mut self, catalog: &WidgetCatalog) {
        let document = &self.document;
        self.selection
            .retain_existing(|node| document.contains(node));
        self.validate(catalog);
    }

    fn validate(&mut self, catalog: &WidgetCatalog) {
        self.diagnostics = CatalogValidator::new(catalog).validate(&self.document);
    }
}
