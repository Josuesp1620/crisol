use crisol_catalog::naming::NodeFactory;
use crisol_commands::Command;
use crisol_commands::commands::{InsertNodeCommand, RemoveNodeCommand, Transaction};
use crisol_engine::dragging::DropTarget;
use crisol_model::identity::NodeId;
use gpui::Context;

use super::{BottomTab, DesignerWorkspace};

/// Changing the document: commands, undo, selection and adding controls.
impl DesignerWorkspace {
    pub fn execute(&mut self, command: &dyn Command, cx: &mut Context<Self>) -> bool {
        let catalog = self.catalog.clone();
        let applied = match self.session_mut().execute(command, &catalog) {
            Ok(_) => true,
            Err(message) => {
                self.output.failure(message);
                self.bottom_tab = BottomTab::Output;
                false
            }
        };
        cx.notify();
        applied
    }

    pub fn undo(&mut self, cx: &mut Context<Self>) {
        let catalog = self.catalog.clone();
        if let Ok(label) = self.session_mut().undo(&catalog) {
            self.output.info(format!("Undo: {label}"));
        }
        self.property_editor = None;
        cx.notify();
    }

    pub fn redo(&mut self, cx: &mut Context<Self>) {
        let catalog = self.catalog.clone();
        if let Ok(label) = self.session_mut().redo(&catalog) {
            self.output.info(format!("Redo: {label}"));
        }
        self.property_editor = None;
        cx.notify();
    }

    pub fn delete_selection(&mut self, cx: &mut Context<Self>) {
        let root = self.session().document().root_id();
        let mut commands: Vec<Box<dyn Command>> = Vec::new();
        for node in self.session().selection().nodes() {
            if *node != root {
                commands.push(Box::new(RemoveNodeCommand { node: *node }));
            }
        }
        if commands.is_empty() {
            return;
        }
        let count = commands.len();
        self.execute(
            &Transaction::new(&format!("Delete {count} control(s)"), commands),
            cx,
        );
        self.property_editor = None;
    }

    /// Selects `node`, or adds it to the selection when `extend` is set (Ctrl+click).
    pub fn select(&mut self, node: NodeId, extend: bool, cx: &mut Context<Self>) {
        let selection = self.session_mut().selection_mut();
        if extend {
            selection.toggle(node);
        } else if !selection.contains(node) || selection.nodes().len() > 1 {
            selection.select_only(node);
        }
        self.property_editor = None;
        self.focused_property = None;
        cx.notify();
    }

    /// Adds a new control of `kind` where `target` says, then selects it.
    pub fn add_control(&mut self, kind: &str, target: DropTarget, cx: &mut Context<Self>) {
        let Some(schema) = self.catalog.schema(kind) else {
            return;
        };
        let node = NodeFactory::new(&self.catalog, self.session().document()).create(
            schema,
            target.container,
            target.local_point,
        );
        let id = node.id;
        let command = InsertNodeCommand {
            parent: target.container,
            index: target.index,
            node,
        };
        if self.execute(&command, cx) {
            self.session_mut().selection_mut().select_only(id);
        }
    }

    /// Adds a control at the end of the selected container, or next to the selected control.
    pub fn add_control_to_selection(&mut self, kind: &str, cx: &mut Context<Self>) {
        let document = self.session().document();
        let mut container = document.root_id();
        if let Some(selected) = self.session().selection().primary()
            && let Ok(node) = document.node(selected)
        {
            let accepts = self
                .catalog
                .schema(node.kind.as_str())
                .is_some_and(|schema| schema.container.accepts_children());
            container = if accepts {
                selected
            } else {
                node.parent().unwrap_or(container)
            };
        }
        let index = document.children_of(container).len();
        let target = DropTarget {
            container,
            index,
            local_point: crisol_model::geometry::Point::new(16, 16),
            marker: None,
        };
        self.add_control(kind, target, cx);
    }
}
