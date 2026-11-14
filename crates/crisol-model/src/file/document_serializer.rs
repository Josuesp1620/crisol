use serde::Serialize;
use serde_json::ser::{PrettyFormatter, Serializer};

use crate::document::{FormDocument, Node};
use crate::identity::NodeId;
use crate::{FormatSettings, ModelError, ModelResult};

use super::document_file::DocumentFile;
use super::node_record::NodeRecord;

/// Reads and writes `.crisol` text. Writing the same document always gives the same bytes,
/// so Git diffs only show real changes.
pub struct DocumentSerializer {
    settings: FormatSettings,
}

impl DocumentSerializer {
    pub fn new(settings: FormatSettings) -> Self {
        Self { settings }
    }

    pub fn write(&self, document: &FormDocument) -> ModelResult<String> {
        let file = DocumentFile {
            format: self.settings.format_version.to_owned(),
            form: document.form().clone(),
            root: Self::record_of(document, document.root_id())?,
        };
        let indent = " ".repeat(self.settings.indent_width);
        let formatter = PrettyFormatter::with_indent(indent.as_bytes());
        let mut buffer = Vec::new();
        let mut serializer = Serializer::with_formatter(&mut buffer, formatter);
        file.serialize(&mut serializer)?;
        buffer.push(b'\n');
        Ok(String::from_utf8_lossy(&buffer).into_owned())
    }

    pub fn read(&self, text: &str) -> ModelResult<FormDocument> {
        let file: DocumentFile = serde_json::from_str(text)?;
        if file.format != self.settings.format_version {
            return Err(ModelError::UnsupportedFormat {
                found: file.format,
                expected: self.settings.format_version.to_owned(),
            });
        }
        let (root, children) = Self::node_of(file.root);
        let root_id = root.id;
        let mut document = FormDocument::new(file.form, root);
        Self::insert_children(&mut document, root_id, children)?;
        Ok(document)
    }

    fn record_of(document: &FormDocument, id: NodeId) -> ModelResult<NodeRecord> {
        let node = document.node(id)?;
        let mut children = Vec::with_capacity(node.children().len());
        for child in node.children() {
            children.push(Self::record_of(document, *child)?);
        }
        Ok(NodeRecord {
            id: node.id,
            kind: node.kind.clone(),
            name: node.name.clone(),
            note: node.note.clone(),
            properties: node.properties.clone(),
            placement: node.placement.clone(),
            events: node.events.clone(),
            children,
        })
    }

    fn insert_children(
        document: &mut FormDocument,
        parent: NodeId,
        records: Vec<NodeRecord>,
    ) -> ModelResult<()> {
        for (index, record) in records.into_iter().enumerate() {
            let (node, children) = Self::node_of(record);
            let id = node.id;
            document.insert(parent, index, node)?;
            Self::insert_children(document, id, children)?;
        }
        Ok(())
    }

    fn node_of(record: NodeRecord) -> (Node, Vec<NodeRecord>) {
        let mut node = Node::new(record.kind, record.name, record.placement);
        node.id = record.id;
        node.note = record.note;
        node.properties = record.properties;
        node.events = record.events;
        (node, record.children)
    }
}

impl Default for DocumentSerializer {
    fn default() -> Self {
        Self::new(FormatSettings::default())
    }
}
