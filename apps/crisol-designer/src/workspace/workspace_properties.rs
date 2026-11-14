use crisol_catalog::PropertyReader;
use crisol_catalog::schema::PropertyType;
use crisol_commands::Command;
use crisol_commands::commands::{
    BindEventCommand, RenameNodeCommand, SetFormInfoCommand, SetPropertyCommand,
};
use crisol_model::identity::{Identifier, NodeId, TypeName};
use crisol_model::properties::PropertyValue;
use crisol_ui::components::{ComboBox, ComboBoxEvent, TextBox, TextBoxEvent};
use gpui::{AppContext, Context, Focusable, Window};

use super::{DesignerWorkspace, EditorField, PropertyEditor, PropertyKey};

/// Editing the property grid: choosing rows, editing values in place and connecting events.
impl DesignerWorkspace {
    pub fn focus_property(&mut self, key: PropertyKey, cx: &mut Context<Self>) {
        self.focused_property = Some(key);
        cx.notify();
    }

    /// Starts editing a row. Yes/no values flip at once and empty events connect at once.
    pub fn begin_edit(&mut self, key: &PropertyKey, window: &mut Window, cx: &mut Context<Self>) {
        self.focused_property = Some(key.clone());
        let Some(node) = self.session().selection().primary() else {
            return;
        };
        match key {
            PropertyKey::Property(name) => match self.property_type(node, name) {
                Some(PropertyType::Boolean) => self.flip_boolean(node, name, cx),
                Some(PropertyType::Choice(options)) => {
                    self.edit_choice(key.clone(), node, name, options, window, cx);
                }
                Some(_) => {
                    let value = self.current_text(node, key);
                    self.edit_text(key.clone(), &value, window, cx);
                }
                None => {}
            },
            PropertyKey::Event(event) => {
                let bound = self
                    .session()
                    .document()
                    .node(node)
                    .ok()
                    .and_then(|found| found.events.handler_for(event).cloned());
                if bound.is_some() {
                    let value = self.current_text(node, key);
                    self.edit_text(key.clone(), &value, window, cx);
                } else {
                    self.bind_default_handler(node, event, cx);
                }
            }
            _ => {
                let value = self.current_text(node, key);
                self.edit_text(key.clone(), &value, window, cx);
            }
        }
        cx.notify();
    }

    /// Connects `event` to the method name Crisol suggests for it.
    pub(super) fn bind_default_handler(
        &mut self,
        node: NodeId,
        event: &str,
        cx: &mut Context<Self>,
    ) {
        let handler = self.default_handler(node, event);
        self.apply(
            &BindEventCommand {
                node,
                event: event.to_owned(),
                handler: Some(handler),
            },
            cx,
        );
    }

    pub fn cancel_edit(&mut self, cx: &mut Context<Self>) {
        self.property_editor = None;
        cx.notify();
    }

    fn commit_edit(&mut self, cx: &mut Context<Self>) {
        let Some(editor) = self.property_editor.take() else {
            return;
        };
        let Some(node) = self.session().selection().primary() else {
            return;
        };
        let text = match &editor.field {
            EditorField::Text(field) => field.read(cx).value().to_owned(),
            EditorField::Choice(field) => field
                .read(cx)
                .selected_text()
                .unwrap_or_default()
                .to_owned(),
        };
        match self.command_for(node, &editor.key, text.trim()) {
            Ok(Some(command)) => {
                self.apply(command.as_ref(), cx);
            }
            Ok(None) => {}
            Err(message) => self.output.failure(message),
        }
        cx.notify();
    }

    fn apply(&mut self, command: &dyn Command, cx: &mut Context<Self>) {
        self.execute(command, cx);
    }

    fn edit_text(
        &mut self,
        key: PropertyKey,
        value: &str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let field = cx.new(|cx| TextBox::new(cx).text(value));
        let subscription = cx.subscribe_in(
            &field,
            window,
            |workspace, _, event: &TextBoxEvent, _, cx| match event {
                TextBoxEvent::Submitted | TextBoxEvent::FocusLost => workspace.commit_edit(cx),
                TextBoxEvent::Changed | TextBoxEvent::FocusGained => {}
            },
        );
        window.focus(&field.focus_handle(cx), cx);
        self.property_editor = Some(PropertyEditor::new(
            key,
            EditorField::Text(field),
            subscription,
        ));
    }

    fn edit_choice(
        &mut self,
        key: PropertyKey,
        node: NodeId,
        name: &str,
        options: &'static [&'static str],
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let current = self.current_text(node, &PropertyKey::Property(name.to_owned()));
        let field = cx.new(|cx| ComboBox::new(cx, options.to_vec()).selected(&current));
        let subscription = cx.subscribe_in(
            &field,
            window,
            |workspace, _, event: &ComboBoxEvent, _, cx| match event {
                ComboBoxEvent::SelectionChanged => workspace.commit_edit(cx),
                ComboBoxEvent::FocusLost => workspace.cancel_edit(cx),
                ComboBoxEvent::Opened | ComboBoxEvent::FocusGained => {}
            },
        );
        self.property_editor = Some(PropertyEditor::new(
            key,
            EditorField::Choice(field),
            subscription,
        ));
    }

    fn flip_boolean(&mut self, node: NodeId, name: &str, cx: &mut Context<Self>) {
        let Ok(found) = self.session().document().node(node) else {
            return;
        };
        let current =
            PropertyReader::new(found, self.catalog.schema(found.kind.as_str())).boolean(name);
        let command = SetPropertyCommand {
            node,
            property: name.to_owned(),
            value: Some(PropertyValue::Boolean(!current)),
        };
        self.apply(&command, cx);
    }

    /// The text shown in a row, which is also what its editor starts with.
    pub fn current_text(&self, node: NodeId, key: &PropertyKey) -> String {
        let document = self.session().document();
        let form = document.form();
        let Ok(found) = document.node(node) else {
            return String::new();
        };
        match key {
            PropertyKey::Name => found.name.to_string(),
            PropertyKey::FormTitle => form.title.clone(),
            PropertyKey::FormType => form.name.to_string(),
            PropertyKey::FormWidth => form.size.width.to_string(),
            PropertyKey::FormHeight => form.size.height.to_string(),
            PropertyKey::Event(event) => found
                .events
                .handler_for(event)
                .map(ToString::to_string)
                .unwrap_or_default(),
            PropertyKey::Property(name) => {
                let reader = PropertyReader::new(found, self.catalog.schema(found.kind.as_str()));
                match self.property_type(node, name) {
                    Some(PropertyType::Integer { .. }) => reader.integer(name).to_string(),
                    Some(PropertyType::TextList) => reader.list(name).join(", "),
                    Some(PropertyType::Boolean) => {
                        if reader.boolean(name) { "Yes" } else { "No" }.to_owned()
                    }
                    _ => reader.text(name),
                }
            }
        }
    }

    fn property_type(&self, node: NodeId, name: &str) -> Option<PropertyType> {
        let found = self.session().document().node(node).ok()?;
        let schema = self.catalog.schema(found.kind.as_str())?;
        Some(schema.property(name)?.value_type.clone())
    }

    fn command_for(
        &self,
        node: NodeId,
        key: &PropertyKey,
        text: &str,
    ) -> Result<Option<Box<dyn Command>>, String> {
        if text == self.current_text(node, key) {
            return Ok(None);
        }
        let form = self.session().document().form().clone();
        let command: Box<dyn Command> = match key {
            PropertyKey::Name => Box::new(RenameNodeCommand {
                node,
                name: Identifier::parse(text).map_err(|error| error.to_string())?,
            }),
            PropertyKey::Event(event) => Box::new(BindEventCommand {
                node,
                event: event.clone(),
                handler: if text.is_empty() {
                    None
                } else {
                    Some(Identifier::parse(text).map_err(|error| error.to_string())?)
                },
            }),
            PropertyKey::FormTitle => {
                let mut changed = form;
                text.clone_into(&mut changed.title);
                Box::new(SetFormInfoCommand { form: changed })
            }
            PropertyKey::FormType => {
                let mut changed = form;
                changed.name = TypeName::parse(text).map_err(|error| error.to_string())?;
                Box::new(SetFormInfoCommand { form: changed })
            }
            PropertyKey::FormWidth | PropertyKey::FormHeight => {
                let pixels: i32 = text
                    .parse()
                    .map_err(|_| format!("{text} is not a whole number."))?;
                let mut changed = form;
                if *key == PropertyKey::FormWidth {
                    changed.size.width = pixels.max(120);
                } else {
                    changed.size.height = pixels.max(80);
                }
                Box::new(SetFormInfoCommand { form: changed })
            }
            PropertyKey::Property(name) => Box::new(SetPropertyCommand {
                node,
                property: name.clone(),
                value: Some(self.parse_value(node, name, text)?),
            }),
        };
        Ok(Some(command))
    }

    fn parse_value(&self, node: NodeId, name: &str, text: &str) -> Result<PropertyValue, String> {
        match self.property_type(node, name) {
            Some(PropertyType::Integer { minimum, maximum }) => {
                let number: i64 = text
                    .parse()
                    .map_err(|_| format!("{text} is not a whole number."))?;
                if (minimum..=maximum).contains(&number) {
                    Ok(PropertyValue::Integer(number))
                } else {
                    Err(format!("{name} must be between {minimum} and {maximum}."))
                }
            }
            Some(PropertyType::TextList) => {
                let mut items = Vec::new();
                for item in text.split(',') {
                    let trimmed = item.trim();
                    if !trimmed.is_empty() {
                        items.push(trimmed.to_owned());
                    }
                }
                Ok(PropertyValue::List(items))
            }
            _ => Ok(PropertyValue::Text(text.to_owned())),
        }
    }

    /// `btn_save` + `click` gives `save`; other events add their name: `save_focus_lost`.
    fn default_handler(&self, node: NodeId, event: &str) -> Identifier {
        let document = self.session().document();
        let (base, default_event) = match document.node(node) {
            Ok(found) => {
                let name = found.name.to_string();
                let base = match name.split_once('_') {
                    Some((_, rest)) if !rest.is_empty() => rest.to_owned(),
                    _ => name,
                };
                let default_event = self
                    .catalog
                    .schema(found.kind.as_str())
                    .and_then(|schema| schema.default_event);
                (base, default_event)
            }
            Err(_) => ("handler".to_owned(), None),
        };
        let candidate = if default_event == Some(event) {
            base.clone()
        } else {
            format!("{base}_{event}")
        };
        Identifier::parse(&candidate).unwrap_or_else(|_| {
            Identifier::parse(&format!("on_{candidate}"))
                .unwrap_or_else(|_| Identifier::parse("handler").expect("a valid name"))
        })
    }
}
