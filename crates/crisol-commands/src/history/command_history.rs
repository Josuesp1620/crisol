use crisol_model::document::FormDocument;

use super::HistoryEntry;
use crate::{Command, CommandError, CommandResult};

/// Applies commands to a document and keeps what is needed to undo and redo them.
#[derive(Debug, Default)]
pub struct CommandHistory {
    undo_stack: Vec<HistoryEntry>,
    redo_stack: Vec<HistoryEntry>,
    /// Number of steps applied since the document was opened, moved by undo and redo.
    position: usize,
    saved_position: Option<usize>,
}

impl CommandHistory {
    pub fn new() -> Self {
        Self {
            saved_position: Some(0),
            ..Self::default()
        }
    }

    pub fn execute(
        &mut self,
        command: &dyn Command,
        document: &mut FormDocument,
    ) -> CommandResult<()> {
        let reverse = command.apply(document)?;
        self.undo_stack.push(HistoryEntry {
            label: command.label(),
            reverse,
        });
        self.redo_stack.clear();
        if self
            .saved_position
            .is_some_and(|saved| saved > self.position)
        {
            self.saved_position = None;
        }
        self.position += 1;
        Ok(())
    }

    pub fn undo(&mut self, document: &mut FormDocument) -> CommandResult<String> {
        let entry = self.undo_stack.pop().ok_or(CommandError::NothingToUndo)?;
        let forward = entry.reverse.apply(document)?;
        self.position -= 1;
        self.redo_stack.push(HistoryEntry {
            label: entry.label.clone(),
            reverse: forward,
        });
        Ok(entry.label)
    }

    pub fn redo(&mut self, document: &mut FormDocument) -> CommandResult<String> {
        let entry = self.redo_stack.pop().ok_or(CommandError::NothingToRedo)?;
        let reverse = entry.reverse.apply(document)?;
        self.position += 1;
        self.undo_stack.push(HistoryEntry {
            label: entry.label.clone(),
            reverse,
        });
        Ok(entry.label)
    }

    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    /// Applied steps, the most recent first.
    pub fn recent_entries(&self) -> impl Iterator<Item = &HistoryEntry> {
        self.undo_stack.iter().rev()
    }

    pub fn mark_saved(&mut self) {
        self.saved_position = Some(self.position);
    }

    pub fn has_unsaved_changes(&self) -> bool {
        self.saved_position != Some(self.position)
    }
}
