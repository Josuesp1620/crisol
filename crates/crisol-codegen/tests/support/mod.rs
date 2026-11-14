use crisol_catalog::WidgetCatalog;
use crisol_catalog::naming::NodeFactory;
use crisol_model::document::{FormDocument, FormInfo, Node, WidgetKind};
use crisol_model::geometry::{Point, Size};
use crisol_model::identity::{Identifier, NodeId, TypeName};
use crisol_model::layout::Placement;
use crisol_model::properties::PropertyValue;

/// The customer form of the plan: a grid of fields and a row of buttons.
pub struct CustomerForm {
    pub catalog: WidgetCatalog,
    pub document: FormDocument,
}

impl CustomerForm {
    pub fn build() -> Self {
        let form = FormInfo {
            name: TypeName::parse("CustomerForm").unwrap(),
            title: "New customer".to_owned(),
            size: Size::new(520, 372),
            state: None,
        };
        let root = Node::new(
            WidgetKind::new("Form"),
            Identifier::parse("frm_customer").unwrap(),
            Placement::flow(),
        );
        let mut sample = Self {
            catalog: WidgetCatalog::standard(),
            document: FormDocument::new(form, root),
        };
        let root = sample.document.root_id();
        let grid = sample.add("Grid", "grd_fields", root);
        sample.set(grid, "first_column_width", PropertyValue::Integer(104));
        let label = sample.add("Label", "lbl_tax_id", grid);
        sample.set(label, "text", PropertyValue::Text("Tax id".to_owned()));
        let tax_id = sample.add("TextBox", "txt_tax_id", grid);
        sample.set(
            tax_id,
            "placeholder",
            PropertyValue::Text("20123456789".to_owned()),
        );
        sample.set(tax_id, "max_length", PropertyValue::Integer(11));
        sample.bind(tax_id, "changed", "tax_id_changed");
        let kind = sample.add("ComboBox", "cmb_kind", grid);
        sample.set(
            kind,
            "items",
            PropertyValue::List(vec!["Company".to_owned(), "Person".to_owned()]),
        );
        sample.add("CheckBox", "chk_active", grid);
        let buttons = sample.add("Stack", "stk_buttons", root);
        sample.set(
            buttons,
            "direction",
            PropertyValue::Text("horizontal".to_owned()),
        );
        sample.set(buttons, "alignment", PropertyValue::Text("end".to_owned()));
        let cancel = sample.add("Button", "btn_cancel", buttons);
        sample.set(cancel, "text", PropertyValue::Text("Cancel".to_owned()));
        let save = sample.add("Button", "btn_save", buttons);
        sample.set(save, "text", PropertyValue::Text("Save \"now\"".to_owned()));
        sample.set(save, "variant", PropertyValue::Text("primary".to_owned()));
        sample.bind(save, "click", "save");
        sample
    }

    pub fn add(&mut self, kind: &str, name: &str, parent: NodeId) -> NodeId {
        let schema = self.catalog.schema(kind).unwrap();
        let mut node = NodeFactory::new(&self.catalog, &self.document).create(
            schema,
            parent,
            Point::new(0, 0),
        );
        node.name = Identifier::parse(name).unwrap();
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

    pub fn bind(&mut self, node: NodeId, event: &str, handler: &str) {
        self.document
            .node_mut(node)
            .unwrap()
            .events
            .bind(event, Identifier::parse(handler).unwrap());
    }
}
