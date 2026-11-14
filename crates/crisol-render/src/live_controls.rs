use std::collections::HashMap;

use crisol_catalog::{PropertyReader, WidgetCatalog};
use crisol_model::document::{DocumentWalker, FormDocument, Node};
use crisol_model::identity::NodeId;
use crisol_ui::components::text_box::InputKind;
use crisol_ui::components::{CheckBox, ComboBox, RadioButton, TextBox};
use gpui::{AnyView, App, AppContext};

/// Working controls for every stateful node, so a preview window can be typed into.
#[derive(Default)]
pub struct LiveControls {
    views: HashMap<NodeId, AnyView>,
}

impl LiveControls {
    pub fn build(document: &FormDocument, catalog: &WidgetCatalog, cx: &mut App) -> Self {
        let mut views = HashMap::new();
        for id in DocumentWalker::new(document).all() {
            let Ok(node) = document.node(id) else {
                continue;
            };
            let reader = PropertyReader::new(node, catalog.schema(node.kind.as_str()));
            if let Some(view) = Self::create(node, &reader, cx) {
                views.insert(id, view);
            }
        }
        Self { views }
    }

    pub fn view_for(&self, node: NodeId) -> Option<&AnyView> {
        self.views.get(&node)
    }

    fn create(node: &Node, reader: &PropertyReader<'_>, cx: &mut App) -> Option<AnyView> {
        let disabled = !reader.boolean("enabled");
        let text = reader.text("text");
        let view: AnyView = match node.kind.as_str() {
            "TextBox" => {
                let placeholder = reader.text("placeholder");
                let input_kind = Self::input_kind(&reader.text("input_kind"));
                let max_length = usize::try_from(reader.integer("max_length")).unwrap_or(0);
                let read_only = reader.boolean("read_only");
                cx.new(|cx| {
                    TextBox::new(cx)
                        .input_kind(input_kind)
                        .max_length(max_length)
                        .text(&text)
                        .placeholder(placeholder)
                        .read_only(read_only)
                        .disabled(disabled)
                })
                .into()
            }
            "CheckBox" => {
                let checked = reader.boolean("checked");
                cx.new(|cx| CheckBox::new(cx, text).checked(checked).disabled(disabled))
                    .into()
            }
            "RadioButton" => {
                let checked = reader.boolean("checked");
                cx.new(|cx| {
                    RadioButton::new(cx, text)
                        .checked(checked)
                        .disabled(disabled)
                })
                .into()
            }
            "ComboBox" => {
                let items = reader.list("items");
                let selected = reader.text("selected");
                let placeholder = reader.text("placeholder");
                cx.new(|cx| {
                    let mut borrowed = Vec::with_capacity(items.len());
                    for item in &items {
                        borrowed.push(item.as_str());
                    }
                    ComboBox::new(cx, borrowed)
                        .selected(&selected)
                        .placeholder(placeholder)
                        .disabled(disabled)
                })
                .into()
            }
            _ => return None,
        };
        Some(view)
    }

    fn input_kind(name: &str) -> InputKind {
        match name {
            "email" => InputKind::Email,
            "number" => InputKind::Number,
            "password" => InputKind::Password,
            "multiline" => InputKind::Multiline,
            _ => InputKind::Text,
        }
    }
}
