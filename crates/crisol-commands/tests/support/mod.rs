use crisol_model::document::{FormDocument, FormInfo, Node, WidgetKind};
use crisol_model::geometry::Size;
use crisol_model::identity::{Identifier, NodeId, TypeName};
use crisol_model::layout::Placement;

/// A form with one stack holding a button.
#[allow(
    dead_code,
    reason = "each test file uses a different part of the sample"
)]
pub struct SampleForm {
    pub document: FormDocument,
    pub stack: NodeId,
    pub button: NodeId,
}

#[allow(
    dead_code,
    reason = "each test file uses a different part of the sample"
)]
impl SampleForm {
    pub fn build() -> Self {
        let form = FormInfo {
            name: TypeName::parse("CustomerForm").unwrap(),
            title: "New customer".to_owned(),
            size: Size::new(520, 372),
            state: None,
        };
        let mut document = FormDocument::new(form, SampleForm::node("Form", "frm_customer"));
        let root = document.root_id();
        let stack = SampleForm::node("Stack", "stk_fields");
        let stack_id = stack.id;
        document.insert(root, 0, stack).unwrap();
        let button = SampleForm::node("Button", "btn_save");
        let button_id = button.id;
        document.insert(stack_id, 0, button).unwrap();
        Self {
            document,
            stack: stack_id,
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
