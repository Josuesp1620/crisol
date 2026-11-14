use crisol_model::document::{FormDocument, FormInfo, Node, WidgetKind};
use crisol_model::geometry::Size;
use crisol_model::identity::{Identifier, NodeId, TypeName};
use crisol_model::layout::Placement;
use crisol_model::properties::PropertyValue;

/// A small customer form: a stack with a text box and a button.
#[allow(
    dead_code,
    reason = "each test file uses a different part of the sample"
)]
pub struct SampleForm {
    pub document: FormDocument,
    pub text_box: NodeId,
    pub button: NodeId,
}

impl SampleForm {
    pub fn build() -> Self {
        let form = FormInfo {
            name: TypeName::parse("CustomerForm").unwrap(),
            title: "New customer".to_owned(),
            size: Size::new(520, 372),
            state: None,
        };
        let root = SampleForm::node("Stack", "stk_root");
        let root_id = root.id;
        let mut document = FormDocument::new(form, root);

        let mut text_box = SampleForm::node("TextBox", "txt_tax_id");
        text_box
            .properties
            .set("placeholder", PropertyValue::Text("20123456789".to_owned()));
        let text_box_id = text_box.id;
        document.insert(root_id, 0, text_box).unwrap();

        let mut button = SampleForm::node("Button", "btn_save");
        button
            .properties
            .set("text", PropertyValue::Text("Save".to_owned()));
        button
            .events
            .bind("click", Identifier::parse("save").unwrap());
        let button_id = button.id;
        document.insert(root_id, 1, button).unwrap();

        Self {
            document,
            text_box: text_box_id,
            button: button_id,
        }
    }

    pub fn node(kind: &str, name: &str) -> Node {
        Node::new(
            WidgetKind::new(kind),
            Identifier::parse(name).unwrap(),
            Placement::flow(),
        )
    }
}
