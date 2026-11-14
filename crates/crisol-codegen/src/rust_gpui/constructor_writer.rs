use crisol_model::document::{DocumentWalker, Node};
use crisol_model::identity::NodeId;

use super::generation_context::GenerationContext;
use super::property_reader::PropertyReader;
use super::stateful_kind::StatefulKind;
use crate::writing::{CodeWriter, NameConverter, RustLiteral};

/// Writes the form struct, its stateful controls and the constructor that creates them
/// and connects their events.
pub struct ConstructorWriter<'context> {
    context: &'context GenerationContext<'context>,
    stateful: Vec<NodeId>,
}

impl<'context> ConstructorWriter<'context> {
    pub fn new(context: &'context GenerationContext<'context>) -> Self {
        let mut stateful = Vec::new();
        for id in DocumentWalker::new(context.document).all() {
            if let Ok(node) = context.document.node(id)
                && StatefulKind::is_stateful(node.kind.as_str())
            {
                stateful.push(id);
            }
        }
        Self { context, stateful }
    }

    pub fn write_struct(&self, writer: &mut CodeWriter) {
        let form = self.context.document.form();
        writer.line(&format!("/// {}", form.title));
        writer.block(&format!("pub struct {} {{", form.name), "}", |body| {
            if let Some(state) = &form.state {
                body.line(&format!("pub state: {state},"));
            }
            for id in &self.stateful {
                let node = self.node(*id);
                body.line(&format!("pub {}: Entity<{}>,", node.name, node.kind));
            }
        });
    }

    pub fn write_constructor(&self, writer: &mut CodeWriter) {
        let form = self.context.document.form();
        let state_parameter = match &form.state {
            Some(state) => format!("state: {state}, "),
            None => String::new(),
        };
        writer.block(&format!("impl {} {{", form.name), "}", |body| {
            body.line(&format!("pub const TITLE: &'static str = {};", RustLiteral::string(&form.title)));
            body.line(&format!(
                "pub const DESIGN_SIZE: (f32, f32) = ({}, {});",
                RustLiteral::pixels(i64::from(form.size.width)),
                RustLiteral::pixels(i64::from(form.size.height))
            ));
            body.blank();
            body.block(
                &format!("pub fn new({state_parameter}window: &mut Window, cx: &mut Context<Self>) -> Self {{"),
                "}",
                |constructor| {
                    for id in &self.stateful {
                        self.write_creation(*id, constructor);
                    }
                    for id in &self.stateful {
                        self.write_subscription(*id, constructor);
                    }
                    self.write_fields(constructor);
                },
            );
        });
    }

    fn write_creation(&self, id: NodeId, writer: &mut CodeWriter) {
        let node = self.node(id);
        let reader = PropertyReader::new(node, self.context.catalog.schema(node.kind.as_str()));
        let mut expression = match node.kind.as_str() {
            "TextBox" => "TextBox::new(cx)".to_owned(),
            "CheckBox" => format!(
                "CheckBox::new(cx, {})",
                RustLiteral::string(&reader.text("text"))
            ),
            "RadioButton" => format!(
                "RadioButton::new(cx, {})",
                RustLiteral::string(&reader.text("text"))
            ),
            _ => format!(
                "ComboBox::new(cx, {})",
                RustLiteral::string_list(&reader.list("items"))
            ),
        };
        expression.push_str(&Self::builder_calls(node.kind.as_str(), &reader));
        writer.line(&format!("let {} = cx.new(|cx| {expression});", node.name));
    }

    fn builder_calls(kind: &str, reader: &PropertyReader<'_>) -> String {
        let mut calls = String::new();
        let mut text_call = |method: &str, property: &str| {
            let value = reader.text(property);
            if !value.is_empty() {
                calls.push_str(&format!(".{method}({})", RustLiteral::string(&value)));
            }
        };
        match kind {
            "TextBox" => {
                text_call("text", "text");
                text_call("placeholder", "placeholder");
            }
            "ComboBox" => {
                text_call("selected", "selected");
                text_call("placeholder", "placeholder");
            }
            _ => {}
        }
        if kind == "TextBox" {
            let input_kind = reader.text("input_kind");
            if input_kind != "text" {
                calls.push_str(&format!(
                    ".input_kind(InputKind::{})",
                    NameConverter::pascal_case(&input_kind)
                ));
            }
            if reader.integer("max_length") > 0 {
                calls.push_str(&format!(".max_length({})", reader.integer("max_length")));
            }
            if reader.boolean("read_only") {
                calls.push_str(".read_only(true)");
            }
        }
        if (kind == "CheckBox" || kind == "RadioButton") && reader.boolean("checked") {
            calls.push_str(".checked(true)");
        }
        if !reader.boolean("enabled") {
            calls.push_str(".disabled(true)");
        }
        calls
    }

    fn write_subscription(&self, id: NodeId, writer: &mut CodeWriter) {
        let node = self.node(id);
        let mut arms: Vec<(String, Vec<String>)> = Vec::new();
        for binding in self.context.bindings {
            if binding.node == id {
                let variant = NameConverter::pascal_case(&binding.event);
                arms.push((
                    variant,
                    vec![format!("this.{}(window, cx);", binding.handler)],
                ));
            }
        }
        if node.kind.as_str() == "RadioButton" {
            self.add_group_exclusion(node, &mut arms);
        }
        if arms.is_empty() {
            return;
        }
        let event_type = StatefulKind::event_type(node.kind.as_str());
        writer.block(
            &format!(
                "cx.subscribe_in(&{}, window, |this, _, event: &{event_type}, window, cx| {{",
                node.name
            ),
            "})",
            |closure| {
                closure.block("match event {", "}", |matcher| {
                    for (variant, statements) in &arms {
                        matcher.block(&format!("{event_type}::{variant} => {{"), "}", |arm| {
                            for statement in statements {
                                arm.line(statement);
                            }
                        });
                    }
                    if !self.covers_every_event(node, arms.len()) {
                        matcher.line("_ => {}");
                    }
                });
            },
        );
        writer.line(".detach();");
    }

    /// Radio buttons of the same group uncheck each other.
    fn add_group_exclusion(&self, node: &Node, arms: &mut Vec<(String, Vec<String>)>) {
        let schema = self.context.catalog.schema("RadioButton");
        let group = PropertyReader::new(node, schema).text("group");
        let mut statements = Vec::new();
        for other_id in &self.stateful {
            let other = self.node(*other_id);
            if other.id != node.id
                && other.kind.as_str() == "RadioButton"
                && PropertyReader::new(other, schema).text("group") == group
            {
                statements.push(format!(
                    "this.{}.update(cx, |radio, cx| radio.set_checked(false, cx));",
                    other.name
                ));
            }
        }
        if statements.is_empty() {
            return;
        }
        match arms.iter_mut().find(|(variant, _)| variant == "Checked") {
            Some((_, existing)) => {
                statements.append(existing);
                *existing = statements;
            }
            None => arms.push(("Checked".to_owned(), statements)),
        }
    }

    fn covers_every_event(&self, node: &Node, handled: usize) -> bool {
        match self.context.catalog.schema(node.kind.as_str()) {
            Some(schema) => schema.events.len() == handled,
            None => false,
        }
    }

    fn write_fields(&self, writer: &mut CodeWriter) {
        let form = self.context.document.form();
        writer.block("Self {", "}", |fields| {
            if form.state.is_some() {
                fields.line("state,");
            }
            for id in &self.stateful {
                fields.line(&format!("{},", self.node(*id).name));
            }
        });
    }

    fn node(&self, id: NodeId) -> &Node {
        self.context
            .document
            .node(id)
            .expect("stateful nodes come from the same document")
    }
}
