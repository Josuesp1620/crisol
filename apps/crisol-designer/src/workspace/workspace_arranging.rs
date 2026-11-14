use crisol_commands::Command;
use crisol_commands::commands::{SetPlacementCommand, Transaction};
use crisol_engine::editing::SubtreeCloner;
use crisol_model::geometry::Point;
use crisol_model::identity::NodeId;
use crisol_model::layout::Placement;
use gpui::Context;

use super::DesignerWorkspace;

/// Moving with the keyboard, the clipboard and walking up the tree.
impl DesignerWorkspace {
    /// Moves the selected absolute controls by `dx`, `dy` logical pixels.
    pub fn nudge(&mut self, dx: i32, dy: i32, cx: &mut Context<Self>) {
        let document = self.session().document();
        let mut commands: Vec<Box<dyn Command>> = Vec::new();
        for node in self.session().selection().nodes() {
            if let Ok(found) = document.node(*node)
                && let Placement::Absolute { rect, anchor } = &found.placement
            {
                commands.push(Box::new(SetPlacementCommand {
                    node: *node,
                    placement: Placement::Absolute {
                        rect: rect.moved_to(rect.origin.offset_by(Point::new(dx, dy))),
                        anchor: anchor.clone(),
                    },
                }));
            }
        }
        if !commands.is_empty() {
            self.execute(&Transaction::new("Move with the keyboard", commands), cx);
        }
    }

    pub fn copy_selection(&mut self, cx: &mut Context<Self>) {
        let root = self.session().document().root_id();
        let mut nodes = Vec::new();
        for node in self.session().selection().nodes() {
            if *node != root {
                nodes.push(*node);
            }
        }
        if nodes.is_empty() {
            return;
        }
        let count = nodes.len();
        self.clipboard = Some((self.session().document().clone(), nodes));
        self.output.info(format!("Copied {count} control(s)."));
        cx.notify();
    }

    /// Pastes into the selected container, or next to the selected control.
    pub fn paste_clipboard(&mut self, cx: &mut Context<Self>) {
        let Some((origin, nodes)) = self.clipboard.clone() else {
            return;
        };
        let target = self.paste_target();
        let pasted = SubtreeCloner::new(self.session().document()).paste(&origin, &nodes, target);
        if let Some((transaction, roots)) = pasted
            && self.execute(&transaction, cx)
        {
            let selection = self.session_mut().selection_mut();
            selection.clear();
            for root in roots {
                selection.toggle(root);
            }
            cx.notify();
        }
    }

    pub fn duplicate_selection(&mut self, cx: &mut Context<Self>) {
        self.copy_selection(cx);
        self.paste_clipboard(cx);
    }

    pub fn select_parent(&mut self, cx: &mut Context<Self>) {
        let document = self.session().document();
        if let Some(selected) = self.session().selection().primary()
            && let Some(parent) = document.parent_of(selected)
        {
            self.select(parent, false, cx);
        }
    }

    fn paste_target(&self) -> NodeId {
        let document = self.session().document();
        let root = document.root_id();
        let Some(selected) = self.session().selection().primary() else {
            return root;
        };
        let Ok(node) = document.node(selected) else {
            return root;
        };
        let accepts = self
            .catalog
            .schema(node.kind.as_str())
            .is_some_and(|schema| schema.container.accepts_children());
        if accepts {
            selected
        } else {
            node.parent().unwrap_or(root)
        }
    }
}
