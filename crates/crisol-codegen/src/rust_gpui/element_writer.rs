use crisol_catalog::widgets::FormSchema;
use crisol_model::document::Node;
use crisol_model::identity::NodeId;
use crisol_model::layout::{Edge, Placement, SizeMode};
use crisol_model::properties::PropertyValue;

use super::generation_context::GenerationContext;
use super::stateful_kind::StatefulKind;
use crate::writing::{CodeWriter, NameConverter, RustLiteral};
use crisol_catalog::PropertyReader;

/// Writes the element tree returned by `render`, one builder expression per control.
pub struct ElementWriter<'context> {
    context: &'context GenerationContext<'context>,
}

impl<'context> ElementWriter<'context> {
    pub fn new(context: &'context GenerationContext<'context>) -> Self {
        Self { context }
    }

    pub fn write_render(&self, writer: &mut CodeWriter) {
        let form = self.context.document.form().name.as_str();
        writer.block(&format!("impl Render for {form} {{"), "}", |body| {
            body.block(
                "fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {",
                "}",
                |render| self.write_element(self.context.document.root_id(), render),
            );
        });
    }

    fn write_element(&self, id: NodeId, writer: &mut CodeWriter) {
        let node = self.node(id);
        let reader = PropertyReader::new(node, self.context.catalog.schema(node.kind.as_str()));
        if StatefulKind::is_stateful(node.kind.as_str()) {
            writer.line(&format!("self.{}.clone()", node.name));
            return;
        }
        match node.kind.as_str() {
            "Label" => writer.line(&format!(
                "Label::new({})",
                RustLiteral::string(&reader.text("text"))
            )),
            "Button" => self.write_button(node, &reader, writer),
            "Grid" => {
                writer.line(&format!("Grid::new({})", reader.integer("columns")));
                Self::write_grid_options(&reader, writer);
                self.write_children(node, writer);
            }
            "Group" => {
                writer.line(&format!(
                    "Group::new({})",
                    RustLiteral::string(&reader.text("title"))
                ));
                Self::write_spacing(&reader, writer);
                self.write_children(node, writer);
            }
            "Panel" => {
                let (width, height) = self.panel_extent(node);
                writer.line(&format!(
                    "Panel::new({}, {})",
                    RustLiteral::pixels(width),
                    RustLiteral::pixels(height)
                ));
                self.write_children(node, writer);
            }
            kind => {
                let direction = if reader.text("direction") == "horizontal" {
                    "horizontal"
                } else {
                    "vertical"
                };
                writer.line(&format!("Stack::{direction}()"));
                writer.indent();
                if kind == FormSchema::KIND {
                    writer.line(".fill()");
                }
                let alignment = reader.text("alignment");
                if !alignment.is_empty() && alignment != "start" {
                    writer.line(&format!(
                        ".align(StackAlignment::{})",
                        NameConverter::pascal_case(&alignment)
                    ));
                }
                writer.dedent();
                Self::write_spacing(&reader, writer);
                self.write_children(node, writer);
            }
        }
    }

    fn write_button(&self, node: &Node, reader: &PropertyReader<'_>, writer: &mut CodeWriter) {
        writer.line(&format!(
            "Button::new({}, {})",
            RustLiteral::string(node.name.as_str()),
            RustLiteral::string(&reader.text("text"))
        ));
        writer.indent();
        let variant = reader.text("variant");
        if variant != "secondary" {
            writer.line(&format!(
                ".variant(ButtonVariant::{})",
                NameConverter::pascal_case(&variant)
            ));
        }
        if !reader.boolean("enabled") {
            writer.line(".disabled(true)");
        }
        for binding in self.context.bindings {
            if binding.node == node.id && binding.event == "click" {
                writer.line(&format!(
                    ".on_click(cx.listener(|this, _, window, cx| this.{}(window, cx)))",
                    binding.handler
                ));
            }
        }
        writer.dedent();
    }

    fn write_spacing(reader: &PropertyReader<'_>, writer: &mut CodeWriter) {
        writer.indent();
        writer.line(&format!(
            ".gap({})",
            RustLiteral::pixels(reader.integer("spacing"))
        ));
        if reader.integer("padding") > 0 {
            writer.line(&format!(
                ".padding({})",
                RustLiteral::pixels(reader.integer("padding"))
            ));
        }
        writer.dedent();
    }

    fn write_grid_options(reader: &PropertyReader<'_>, writer: &mut CodeWriter) {
        writer.indent();
        if reader.integer("first_column_width") > 0 {
            writer.line(&format!(
                ".first_column_width({})",
                RustLiteral::pixels(reader.integer("first_column_width"))
            ));
        }
        writer.line(&format!(
            ".row_gap({})",
            RustLiteral::pixels(reader.integer("row_spacing"))
        ));
        writer.dedent();
        Self::write_spacing(reader, writer);
    }

    fn write_children(&self, node: &Node, writer: &mut CodeWriter) {
        writer.indent();
        for child_id in node.children() {
            let child = self.node(*child_id);
            if child.properties.get("visible") == Some(&PropertyValue::Boolean(false)) {
                continue;
            }
            writer.line(&format!(".{}", Self::slot_call(&child.placement)));
            writer.indent();
            self.write_element(*child_id, writer);
            writer.dedent();
            writer.line(")");
        }
        writer.dedent();
    }

    /// The container method that receives a child, with its placement arguments.
    fn slot_call(placement: &Placement) -> String {
        match placement {
            Placement::Flow {
                width: SizeMode::Hug,
                height: SizeMode::Hug,
            } => "child(".to_owned(),
            Placement::Flow { width, height } => {
                format!(
                    "child_sized({}, {},",
                    Self::length(*width),
                    Self::length(*height)
                )
            }
            Placement::Cell { cell, width } => format!(
                "cell({}, {}, {}, {},",
                cell.row,
                cell.column,
                cell.column_span,
                Self::length(*width)
            ),
            Placement::Absolute { rect, anchor } => format!(
                "place({}, {}, {}, {}, Anchors::new({}, {}, {}, {}),",
                RustLiteral::pixels(i64::from(rect.left())),
                RustLiteral::pixels(i64::from(rect.top())),
                RustLiteral::pixels(i64::from(rect.size.width)),
                RustLiteral::pixels(i64::from(rect.size.height)),
                anchor.contains(Edge::Top),
                anchor.contains(Edge::Left),
                anchor.contains(Edge::Right),
                anchor.contains(Edge::Bottom)
            ),
        }
    }

    fn length(mode: SizeMode) -> String {
        match mode {
            SizeMode::Fixed(pixels) => {
                format!("Length::Fixed({})", RustLiteral::pixels(i64::from(pixels)))
            }
            SizeMode::Hug => "Length::Hug".to_owned(),
            SizeMode::Fill => "Length::Fill".to_owned(),
        }
    }

    /// An absolute panel is as large as the furthest right and bottom edge of its children.
    fn panel_extent(&self, panel: &Node) -> (i64, i64) {
        let mut width = 0;
        let mut height = 0;
        for child_id in panel.children() {
            if let Placement::Absolute { rect, .. } = &self.node(*child_id).placement {
                width = width.max(i64::from(rect.right()));
                height = height.max(i64::from(rect.bottom()));
            }
        }
        (width, height)
    }

    fn node(&self, id: NodeId) -> &Node {
        self.context
            .document
            .node(id)
            .expect("children come from the same document")
    }
}
