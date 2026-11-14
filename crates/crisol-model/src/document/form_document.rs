use std::collections::HashMap;

use crate::identity::NodeId;
use crate::{ModelError, ModelResult};

use super::{DetachedSubtree, DocumentWalker, FormInfo, Node};

/// A form: its description and the tree of controls under one root container.
///
/// Nodes live in a flat map keyed by id; the tree is made of their parent and children ids.
/// Every structural change goes through the methods here, which keep both directions in sync.
#[derive(Debug, Clone, PartialEq)]
pub struct FormDocument {
    form: FormInfo,
    root: NodeId,
    nodes: HashMap<NodeId, Node>,
}

impl FormDocument {
    pub fn new(form: FormInfo, mut root: Node) -> Self {
        root.parent = None;
        let root_id = root.id;
        let mut nodes = HashMap::new();
        nodes.insert(root_id, root);
        Self {
            form,
            root: root_id,
            nodes,
        }
    }

    pub fn form(&self) -> &FormInfo {
        &self.form
    }

    pub fn form_mut(&mut self) -> &mut FormInfo {
        &mut self.form
    }

    pub fn root_id(&self) -> NodeId {
        self.root
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn contains(&self, id: NodeId) -> bool {
        self.nodes.contains_key(&id)
    }

    pub fn node(&self, id: NodeId) -> ModelResult<&Node> {
        self.nodes.get(&id).ok_or(ModelError::NodeNotFound(id))
    }

    pub fn node_mut(&mut self, id: NodeId) -> ModelResult<&mut Node> {
        self.nodes.get_mut(&id).ok_or(ModelError::NodeNotFound(id))
    }

    pub fn parent_of(&self, id: NodeId) -> Option<NodeId> {
        self.nodes.get(&id).and_then(Node::parent)
    }

    pub fn children_of(&self, id: NodeId) -> &[NodeId] {
        match self.nodes.get(&id) {
            Some(node) => node.children(),
            None => &[],
        }
    }

    /// Position of a node among its parent's children.
    pub fn index_in_parent(&self, id: NodeId) -> Option<usize> {
        let parent = self.parent_of(id)?;
        self.children_of(parent)
            .iter()
            .position(|child| *child == id)
    }

    pub fn find_by_name(&self, name: &str) -> Option<NodeId> {
        for node in self.nodes.values() {
            if node.name.as_str() == name {
                return Some(node.id);
            }
        }
        None
    }

    /// Adds a new node as the child at `index` of `parent`.
    pub fn insert(&mut self, parent: NodeId, index: usize, mut node: Node) -> ModelResult<()> {
        if self.find_by_name(node.name.as_str()).is_some() {
            return Err(ModelError::DuplicateName(node.name.to_string()));
        }
        let length = self.node(parent)?.children.len();
        if index > length {
            return Err(ModelError::ChildIndexOutOfRange { index, length });
        }
        node.parent = Some(parent);
        node.children.clear();
        let id = node.id;
        self.nodes.insert(id, node);
        self.node_mut(parent)?.children.insert(index, id);
        Ok(())
    }

    /// Takes a node and its descendants out of the document.
    pub fn detach(&mut self, id: NodeId) -> ModelResult<DetachedSubtree> {
        if id == self.root {
            return Err(ModelError::RootIsFixed);
        }
        let parent = self.parent_of(id).ok_or(ModelError::NodeNotFound(id))?;
        let index = self
            .index_in_parent(id)
            .ok_or(ModelError::NodeNotFound(id))?;
        let members = DocumentWalker::new(self).subtree(id);
        self.node_mut(parent)?.children.remove(index);
        let mut nodes = Vec::with_capacity(members.len());
        for member in members {
            if let Some(node) = self.nodes.remove(&member) {
                nodes.push(node);
            }
        }
        Ok(DetachedSubtree {
            root: id,
            parent,
            index,
            nodes,
        })
    }

    /// Puts a detached subtree back where it was taken from.
    pub fn reattach(&mut self, subtree: DetachedSubtree) -> ModelResult<()> {
        let length = self.node(subtree.parent)?.children.len();
        if subtree.index > length {
            return Err(ModelError::ChildIndexOutOfRange {
                index: subtree.index,
                length,
            });
        }
        for node in subtree.nodes {
            self.nodes.insert(node.id, node);
        }
        self.node_mut(subtree.parent)?
            .children
            .insert(subtree.index, subtree.root);
        Ok(())
    }

    /// Moves a node under `new_parent` at `index` and returns where it was before.
    pub fn move_node(
        &mut self,
        id: NodeId,
        new_parent: NodeId,
        index: usize,
    ) -> ModelResult<(NodeId, usize)> {
        if id == self.root {
            return Err(ModelError::RootIsFixed);
        }
        if DocumentWalker::new(self).is_within(new_parent, id) {
            return Err(ModelError::CyclicMove);
        }
        let old_parent = self.parent_of(id).ok_or(ModelError::NodeNotFound(id))?;
        let old_index = self
            .index_in_parent(id)
            .ok_or(ModelError::NodeNotFound(id))?;
        self.node_mut(old_parent)?.children.remove(old_index);
        let length = self.node(new_parent)?.children.len();
        if index > length {
            self.node_mut(old_parent)?.children.insert(old_index, id);
            return Err(ModelError::ChildIndexOutOfRange { index, length });
        }
        self.node_mut(new_parent)?.children.insert(index, id);
        self.node_mut(id)?.parent = Some(new_parent);
        Ok((old_parent, old_index))
    }
}
