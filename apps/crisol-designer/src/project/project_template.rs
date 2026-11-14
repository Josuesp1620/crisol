use crisol_catalog::WidgetCatalog;
use crisol_catalog::naming::NodeFactory;
use crisol_model::document::{FormDocument, FormInfo, Node, WidgetKind};
use crisol_model::geometry::{Point, Size};
use crisol_model::identity::{Identifier, NodeId, TypeName};
use crisol_model::layout::{Placement, SizeMode};
use crisol_model::properties::PropertyValue;
use crisol_ui::icon::IconName;

use crate::session::DocumentFiles;

/// What a new project starts with.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ProjectTemplate {
    #[default]
    DesktopApplication,
    DataForm,
    Dialog,
    Empty,
}

impl ProjectTemplate {
    pub const ALL: [ProjectTemplate; 4] = [
        ProjectTemplate::DesktopApplication,
        ProjectTemplate::DataForm,
        ProjectTemplate::Dialog,
        ProjectTemplate::Empty,
    ];

    const DATA_FORM: &'static str = include_str!("../../../../examples/customer/customer.crisol");

    pub fn label(self) -> &'static str {
        match self {
            ProjectTemplate::DesktopApplication => "Desktop application",
            ProjectTemplate::DataForm => "Data form",
            ProjectTemplate::Dialog => "Dialog",
            ProjectTemplate::Empty => "Empty project",
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            ProjectTemplate::DesktopApplication => "A main window with a title and a button",
            ProjectTemplate::DataForm => "Fields in a grid with Save and Cancel",
            ProjectTemplate::Dialog => "A message with OK and Cancel",
            ProjectTemplate::Empty => "A blank main window",
        }
    }

    pub fn icon(self) -> IconName {
        match self {
            ProjectTemplate::DesktopApplication => IconName::Form,
            ProjectTemplate::DataForm => IconName::Grid,
            ProjectTemplate::Dialog => IconName::Group,
            ProjectTemplate::Empty => IconName::Templ,
        }
    }

    /// The main form of a new project called `title`.
    pub fn main_form(self, title: &str, catalog: &WidgetCatalog) -> FormDocument {
        let mut document = match self {
            ProjectTemplate::DataForm => {
                DocumentFiles::parse(Self::DATA_FORM).expect("the data form template is valid")
            }
            _ => Self::blank(),
        };
        let form = document.form_mut();
        form.name = TypeName::parse("MainForm").expect("a valid type name");
        title.clone_into(&mut form.title);
        let root = document.root_id();
        match self {
            ProjectTemplate::DesktopApplication => {
                let heading = Self::add(&mut document, catalog, "Label", root);
                Self::set(
                    &mut document,
                    heading,
                    "text",
                    PropertyValue::Text(format!("Welcome to {title}")),
                );
                let start = Self::add(&mut document, catalog, "Button", root);
                Self::set(
                    &mut document,
                    start,
                    "text",
                    PropertyValue::Text("Start".to_owned()),
                );
                Self::set(
                    &mut document,
                    start,
                    "variant",
                    PropertyValue::Text("primary".to_owned()),
                );
            }
            ProjectTemplate::Dialog => {
                let message = Self::add(&mut document, catalog, "Label", root);
                Self::set(
                    &mut document,
                    message,
                    "text",
                    PropertyValue::Text("Do you want to continue?".to_owned()),
                );
                let row = Self::add(&mut document, catalog, "Stack", root);
                Self::set(
                    &mut document,
                    row,
                    "direction",
                    PropertyValue::Text("horizontal".to_owned()),
                );
                Self::set(
                    &mut document,
                    row,
                    "alignment",
                    PropertyValue::Text("end".to_owned()),
                );
                if let Ok(node) = document.node_mut(row) {
                    node.placement = Placement::Flow {
                        width: SizeMode::Fill,
                        height: SizeMode::Hug,
                    };
                }
                let cancel = Self::add(&mut document, catalog, "Button", row);
                Self::set(
                    &mut document,
                    cancel,
                    "text",
                    PropertyValue::Text("Cancel".to_owned()),
                );
                let accept = Self::add(&mut document, catalog, "Button", row);
                Self::set(
                    &mut document,
                    accept,
                    "text",
                    PropertyValue::Text("OK".to_owned()),
                );
                Self::set(
                    &mut document,
                    accept,
                    "variant",
                    PropertyValue::Text("primary".to_owned()),
                );
                document.form_mut().size = Size::new(380, 180);
            }
            ProjectTemplate::DataForm | ProjectTemplate::Empty => {}
        }
        document
    }

    fn blank() -> FormDocument {
        let form = FormInfo {
            name: TypeName::parse("MainForm").expect("a valid type name"),
            title: String::new(),
            size: Size::new(520, 372),
            state: None,
        };
        let root = Node::new(
            WidgetKind::new("Form"),
            Identifier::parse("frm_main").expect("a valid name"),
            Placement::flow(),
        );
        FormDocument::new(form, root)
    }

    fn add(
        document: &mut FormDocument,
        catalog: &WidgetCatalog,
        kind: &str,
        parent: NodeId,
    ) -> NodeId {
        let schema = catalog.schema(kind).expect("a catalog control");
        let node = NodeFactory::new(catalog, document).create(schema, parent, Point::new(0, 0));
        let id = node.id;
        let index = document.children_of(parent).len();
        document.insert(parent, index, node).expect("a new name");
        id
    }

    fn set(document: &mut FormDocument, node: NodeId, property: &str, value: PropertyValue) {
        if let Ok(found) = document.node_mut(node) {
            found.properties.set(property, value);
        }
    }
}
