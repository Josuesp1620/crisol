use crisol_catalog::WidgetCatalog;
use crisol_catalog::naming::NodeFactory;
use crisol_catalog::widgets::FormSchema;
use crisol_model::document::{FormDocument, FormInfo, Node, WidgetKind};
use crisol_model::geometry::{Point, Size};
use crisol_model::identity::{Identifier, NodeId, TypeName};
use crisol_model::layout::Placement;
use crisol_model::properties::PropertyValue;

/// An empty form plus helpers to add controls the way the designer does.
pub struct SampleDocument {
    pub catalog: WidgetCatalog,
    pub document: FormDocument,
}

#[allow(
    dead_code,
    reason = "each test file uses a different part of the sample"
)]
impl SampleDocument {
    pub fn empty() -> Self {
        let form = FormInfo {
            name: TypeName::parse("CustomerForm").unwrap(),
            title: "New customer".to_owned(),
            size: Size::new(520, 372),
            state: None,
        };
        let root = Node::new(
            WidgetKind::new(FormSchema::KIND),
            Identifier::parse("frm_customer").unwrap(),
            Placement::flow(),
        );
        Self {
            catalog: WidgetCatalog::standard(),
            document: FormDocument::new(form, root),
        }
    }

    pub fn root(&self) -> NodeId {
        self.document.root_id()
    }

    pub fn add(&mut self, kind: &str, parent: NodeId) -> NodeId {
        let schema = self.catalog.schema(kind).unwrap();
        let node = NodeFactory::new(&self.catalog, &self.document).create(
            schema,
            parent,
            Point::new(16, 24),
        );
        let id = node.id;
        let index = self.document.children_of(parent).len();
        self.document.insert(parent, index, node).unwrap();
        id
    }

    pub fn set(&mut self, node: NodeId, property: &str, value: PropertyValue) {
        self.document
            .node_mut(node)
            .unwrap()
            .properties
            .set(property, value);
    }

    pub fn name_of(&self, node: NodeId) -> String {
        self.document.node(node).unwrap().name.to_string()
    }
}
