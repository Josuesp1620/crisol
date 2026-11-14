use std::collections::{HashMap, HashSet};

use crisol_commands::Command;
use crisol_commands::commands::{InsertNodeCommand, Transaction};
use crisol_model::document::{DocumentWalker, FormDocument, Node};
use crisol_model::geometry::Point;
use crisol_model::identity::{Identifier, NodeId};
use crisol_model::layout::Placement;

/// Copies controls with everything inside them: new ids, new unique names and, for
/// absolute positions, a small offset so the copy does not hide the original.
pub struct SubtreeCloner<'document> {
    document: &'document FormDocument,
    taken: HashSet<String>,
}

impl<'document> SubtreeCloner<'document> {
    /// How far a pasted absolute control moves from the original, in logical pixels.
    pub const PASTE_OFFSET: i32 = 16;

    pub fn new(document: &'document FormDocument) -> Self {
        let mut taken = HashSet::new();
        for id in DocumentWalker::new(document).all() {
            if let Ok(node) = document.node(id) {
                taken.insert(node.name.to_string());
            }
        }
        Self { document, taken }
    }

    /// One transaction inserting copies of `sources` (taken from `origin`, which may be
    /// another snapshot of the form) at the end of `target`. Returns it with the copies' ids.
    pub fn paste(
        &mut self,
        origin: &FormDocument,
        sources: &[NodeId],
        target: NodeId,
    ) -> Option<(Transaction, Vec<NodeId>)> {
        let mut commands: Vec<Box<dyn Command>> = Vec::new();
        let mut roots = Vec::new();
        let mut index = self.document.children_of(target).len();
        for source in sources {
            if !origin.contains(*source) || *source == origin.root_id() {
                continue;
            }
            let mut copies: HashMap<NodeId, NodeId> = HashMap::new();
            for member in DocumentWalker::new(origin).subtree(*source) {
                let Ok(original) = origin.node(member) else {
                    continue;
                };
                let copy = self.copy_of(original, member == *source);
                let (parent, position) = if member == *source {
                    let position = index;
                    index += 1;
                    (target, position)
                } else {
                    let parent = original
                        .parent()
                        .and_then(|parent| copies.get(&parent).copied())?;
                    let position = origin.index_in_parent(member).unwrap_or(0);
                    (parent, position)
                };
                copies.insert(member, copy.id);
                if member == *source {
                    roots.push(copy.id);
                }
                commands.push(Box::new(InsertNodeCommand {
                    parent,
                    index: position,
                    node: copy,
                }));
            }
        }
        if commands.is_empty() {
            return None;
        }
        Some((Transaction::new("Paste", commands), roots))
    }

    fn copy_of(&mut self, original: &Node, offset: bool) -> Node {
        let mut placement = original.placement.clone();
        if offset && let Placement::Absolute { rect, .. } = &mut placement {
            *rect = rect.moved_to(
                rect.origin
                    .offset_by(Point::new(Self::PASTE_OFFSET, Self::PASTE_OFFSET)),
            );
        }
        let mut copy = Node::new(
            original.kind.clone(),
            self.free_name(&original.name),
            placement,
        );
        copy.note.clone_from(&original.note);
        copy.properties = original.properties.clone();
        copy.events = original.events.clone();
        copy
    }

    /// `btn_save` becomes `btn_save_2`, then `btn_save_3` and so on.
    fn free_name(&mut self, name: &Identifier) -> Identifier {
        let mut number = 2;
        loop {
            let candidate = format!("{name}_{number}");
            if !self.taken.contains(&candidate)
                && let Ok(identifier) = Identifier::parse(&candidate)
            {
                self.taken.insert(candidate);
                return identifier;
            }
            number += 1;
        }
    }
}
