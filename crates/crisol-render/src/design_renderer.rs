use std::rc::Rc;

use crisol_catalog::schema::ContainerRule;
use crisol_catalog::widgets::FormSchema;
use crisol_catalog::{PropertyReader, WidgetCatalog};
use crisol_model::document::{FormDocument, Node};
use crisol_model::identity::NodeId;
use crisol_model::layout::{Edge, Placement};
use crisol_ui::components::appearance::{
    CheckBoxAppearance, ComboBoxAppearance, RadioButtonAppearance, TextBoxAppearance,
};
use crisol_ui::components::{Button, ButtonVariant, Label};
use crisol_ui::layout::{Anchors, Grid, Group, Length, Panel, Stack, StackAlignment};
use gpui::{AnyElement, IntoElement, ParentElement, SharedString, Styled};

use crate::empty_container::EmptyContainer;
use crate::{LayoutRecorder, LengthConverter, LiveControls};

/// Builds the element tree of a document for the canvas. Controls look exactly as they
/// will at run time but do not react to input; the canvas handles the pointer instead.
pub struct DesignRenderer<'document> {
    document: &'document FormDocument,
    catalog: Rc<WidgetCatalog>,
    recorder: LayoutRecorder,
    live: Option<&'document LiveControls>,
}

impl<'document> DesignRenderer<'document> {
    pub fn new(
        document: &'document FormDocument,
        catalog: Rc<WidgetCatalog>,
        recorder: LayoutRecorder,
    ) -> Self {
        Self {
            document,
            catalog,
            recorder,
            live: None,
        }
    }

    /// Draws working controls instead of their appearance, for a preview window.
    pub fn with_live_controls(mut self, live: &'document LiveControls) -> Self {
        self.live = Some(live);
        self
    }

    pub fn render(&self) -> AnyElement {
        self.element_for(self.document.root_id())
    }

    fn element_for(&self, id: NodeId) -> AnyElement {
        let Ok(node) = self.document.node(id) else {
            return gpui::Empty.into_any_element();
        };
        let reader = PropertyReader::new(node, self.catalog.schema(node.kind.as_str()));
        let mut element = match self.live.and_then(|live| live.view_for(id)) {
            Some(view) => view.clone().into_any_element(),
            None => self.control_for(node, &reader),
        };
        if !reader.boolean("visible") {
            element = gpui::div().opacity(0.4).child(element).into_any_element();
        }
        self.recorder.measure(id, element)
    }

    fn control_for(&self, node: &Node, reader: &PropertyReader<'_>) -> AnyElement {
        let disabled = !reader.boolean("enabled");
        match node.kind.as_str() {
            "Label" => Label::new(reader.text("text")).into_any_element(),
            "Button" => Button::new(
                SharedString::from(node.name.to_string()),
                reader.text("text"),
            )
            .variant(Self::button_variant(&reader.text("variant")))
            .disabled(disabled)
            .into_any_element(),
            "TextBox" => {
                TextBoxAppearance::new(Self::masked(reader), reader.text("placeholder"), disabled)
                    .into_any_element()
            }
            "CheckBox" => {
                CheckBoxAppearance::new(reader.text("text"), reader.boolean("checked"), disabled)
                    .into_any_element()
            }
            "RadioButton" => {
                RadioButtonAppearance::new(reader.text("text"), reader.boolean("checked"), disabled)
                    .into_any_element()
            }
            "ComboBox" => {
                let selected = reader.text("selected");
                let text = if selected.is_empty() {
                    None
                } else {
                    Some(SharedString::from(selected))
                };
                ComboBoxAppearance::new(text, reader.text("placeholder"))
                    .disabled(disabled)
                    .into_any_element()
            }
            _ => self.container_for(node, reader),
        }
    }

    fn container_for(&self, node: &Node, reader: &PropertyReader<'_>) -> AnyElement {
        if node.children().is_empty() && node.kind.as_str() != FormSchema::KIND {
            return EmptyContainer::new(node.kind.to_string()).into_any_element();
        }
        let rule = match self.catalog.schema(node.kind.as_str()) {
            Some(schema) => schema.container,
            None => ContainerRule::Flow,
        };
        #[allow(
            clippy::cast_precision_loss,
            reason = "spacing values are small integers"
        )]
        let (gap, padding) = (
            reader.integer("spacing") as f32,
            reader.integer("padding") as f32,
        );
        match (rule, node.kind.as_str()) {
            (ContainerRule::Grid, _) => self.grid_for(node, reader, gap, padding),
            (ContainerRule::Absolute, _) => self.panel_for(node),
            (_, "Group") => {
                let mut group = Group::new(reader.text("title")).gap(gap).padding(padding);
                for child in self.visible_children(node) {
                    let (width, height) = Self::flow_lengths(child);
                    group = group.child_sized(width, height, self.element_for(child.id));
                }
                group.into_any_element()
            }
            _ => {
                let mut stack = if reader.text("direction") == "horizontal" {
                    Stack::horizontal()
                } else {
                    Stack::vertical()
                };
                stack = stack
                    .gap(gap)
                    .padding(padding)
                    .align(Self::alignment(&reader.text("alignment")));
                for child in self.visible_children(node) {
                    let (width, height) = Self::flow_lengths(child);
                    stack = stack.child_sized(width, height, self.element_for(child.id));
                }
                stack.into_any_element()
            }
        }
    }

    #[allow(
        clippy::cast_precision_loss,
        reason = "layout values are small integers"
    )]
    fn grid_for(
        &self,
        node: &Node,
        reader: &PropertyReader<'_>,
        gap: f32,
        padding: f32,
    ) -> AnyElement {
        let columns = u32::try_from(reader.integer("columns")).unwrap_or(1);
        let mut grid = Grid::new(columns)
            .first_column_width(reader.integer("first_column_width") as f32)
            .row_gap(reader.integer("row_spacing") as f32)
            .gap(gap)
            .padding(padding);
        for child in self.visible_children(node) {
            if let Placement::Cell { cell, width } = &child.placement {
                grid = grid.cell(
                    cell.row,
                    cell.column,
                    cell.column_span,
                    LengthConverter::length(*width),
                    self.element_for(child.id),
                );
            }
        }
        grid.into_any_element()
    }

    #[allow(
        clippy::cast_precision_loss,
        reason = "logical pixels are small integers"
    )]
    fn panel_for(&self, node: &Node) -> AnyElement {
        let mut width = 0;
        let mut height = 0;
        for child in self.visible_children(node) {
            if let Placement::Absolute { rect, .. } = &child.placement {
                width = width.max(rect.right());
                height = height.max(rect.bottom());
            }
        }
        let mut panel = Panel::new(width as f32, height as f32);
        for child in self.visible_children(node) {
            if let Placement::Absolute { rect, anchor } = &child.placement {
                let anchors = Anchors::new(
                    anchor.contains(Edge::Top),
                    anchor.contains(Edge::Left),
                    anchor.contains(Edge::Right),
                    anchor.contains(Edge::Bottom),
                );
                panel = panel.place(
                    rect.left() as f32,
                    rect.top() as f32,
                    rect.size.width as f32,
                    rect.size.height as f32,
                    anchors,
                    self.element_for(child.id),
                );
            }
        }
        panel.into_any_element()
    }

    /// Children in order. Hidden ones stay on the canvas, dimmed, so they can be selected.
    fn visible_children(&self, node: &Node) -> Vec<&Node> {
        let mut children = Vec::with_capacity(node.children().len());
        for child_id in node.children() {
            if let Ok(child) = self.document.node(*child_id) {
                children.push(child);
            }
        }
        children
    }

    #[allow(
        clippy::cast_precision_loss,
        reason = "logical pixels are small integers"
    )]
    fn flow_lengths(child: &Node) -> (Length, Length) {
        match &child.placement {
            Placement::Flow { width, height } => (
                LengthConverter::length(*width),
                LengthConverter::length(*height),
            ),
            Placement::Cell { width, .. } => (LengthConverter::length(*width), Length::Hug),
            Placement::Absolute { rect, .. } => (
                Length::Fixed(f32::from(u16::try_from(rect.size.width).unwrap_or(0))),
                Length::Fixed(f32::from(u16::try_from(rect.size.height).unwrap_or(0))),
            ),
        }
    }

    fn masked(reader: &PropertyReader<'_>) -> String {
        let text = reader.text("text");
        if reader.text("input_kind") == "password" {
            "•".repeat(text.chars().count())
        } else {
            text
        }
    }

    fn button_variant(variant: &str) -> ButtonVariant {
        match variant {
            "primary" => ButtonVariant::Primary,
            "ghost" => ButtonVariant::Ghost,
            _ => ButtonVariant::Secondary,
        }
    }

    fn alignment(alignment: &str) -> StackAlignment {
        match alignment {
            "center" => StackAlignment::Center,
            "end" => StackAlignment::End,
            "space_between" => StackAlignment::SpaceBetween,
            _ => StackAlignment::Start,
        }
    }
}
