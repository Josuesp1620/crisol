use crisol_ui::icon::{Icon, IconName};
use crisol_ui::theme::Theme;
use gpui::{
    AnyElement, Context, FontWeight, InteractiveElement, IntoElement, ParentElement,
    StatefulInteractiveElement, Styled, Window, div, prelude::FluentBuilder, px,
};

use super::{PropertyRow, PropertyRowsBuilder};
use crate::chrome::{IconButton, PaneHeader, ToolBarSeparator};
use crate::designer_texts::DesignerTexts;
use crate::panels::ToolboxPanel;
use crate::workspace::{DesignerWorkspace, EditorField, PropertyTab};

/// The property grid: categories of name and value rows, with a description at the bottom.
pub struct PropertyGridPanel<'workspace> {
    workspace: &'workspace DesignerWorkspace,
}

impl<'workspace> PropertyGridPanel<'workspace> {
    pub fn new(workspace: &'workspace DesignerWorkspace) -> Self {
        Self { workspace }
    }

    pub fn render(self, _window: &mut Window, cx: &mut Context<DesignerWorkspace>) -> AnyElement {
        let theme = Theme::of(cx).clone();
        let mut panel = div()
            .flex()
            .flex_col()
            .flex_1()
            .min_h_0()
            .child(PaneHeader::new(IconName::Props, DesignerTexts::PROPERTIES));
        let Some(node) = self.workspace.session().selection().primary() else {
            return panel
                .child(
                    div()
                        .p(px(12.))
                        .text_color(theme.colors.muted)
                        .child(DesignerTexts::NO_SELECTION),
                )
                .into_any_element();
        };
        let rows = PropertyRowsBuilder::new(self.workspace).rows(node);
        panel = panel
            .child(self.object_selector(&theme, node))
            .child(self.tool_strip(&theme, cx))
            .child(self.grid(&theme, &rows, cx))
            .child(self.description(&theme, &rows));
        panel.into_any_element()
    }

    fn object_selector(
        &self,
        theme: &Theme,
        node: crisol_model::identity::NodeId,
    ) -> impl IntoElement {
        let document = self.workspace.session().document();
        let (name, kind, icon) = match document.node(node) {
            Ok(found) => {
                let schema = self.workspace.catalog().schema(found.kind.as_str());
                let label =
                    schema.map_or_else(|| found.kind.to_string(), |schema| schema.label.to_owned());
                let icon = ToolboxPanel::icon_for(schema.map_or("", |schema| schema.icon));
                (found.name.to_string(), label, icon)
            }
            Err(_) => (String::new(), String::new(), IconName::Form),
        };
        div()
            .flex()
            .items_center()
            .gap(px(8.))
            .mx(px(8.))
            .mt(px(8.))
            .mb(px(6.))
            .px(px(8.))
            .py(px(4.))
            .rounded(theme.metrics.small_radius)
            .border_1()
            .border_color(theme.colors.line)
            .bg(theme.colors.background)
            .child(Icon::new(icon).color(theme.colors.muted))
            .child(
                div()
                    .font_weight(FontWeight::SEMIBOLD)
                    .font_family(theme.fonts.mono.clone())
                    .child(name),
            )
            .child(div().flex_1().text_color(theme.colors.muted).child(kind))
            .child(Icon::new(IconName::Down).color(theme.colors.muted))
    }

    fn tool_strip(&self, theme: &Theme, cx: &mut Context<DesignerWorkspace>) -> impl IntoElement {
        let tab = self.workspace.property_tab();
        div()
            .flex()
            .items_center()
            .gap(px(2.))
            .px(px(8.))
            .pb(px(6.))
            .border_b_1()
            .border_color(theme.colors.line)
            .child(IconButton::new("by-category", IconName::Cats).active(true))
            .child(IconButton::new("alphabetical", IconName::Az))
            .child(ToolBarSeparator)
            .child(
                IconButton::new("properties-tab", IconName::Props)
                    .active(tab == PropertyTab::Properties)
                    .on_click(Box::new(cx.listener(|workspace, _, _, cx| {
                        workspace.show_property_tab(PropertyTab::Properties, cx);
                    }))),
            )
            .child(
                IconButton::new("events-tab", IconName::Bolt)
                    .active(tab == PropertyTab::Events)
                    .on_click(Box::new(cx.listener(|workspace, _, _, cx| {
                        workspace.show_property_tab(PropertyTab::Events, cx);
                    }))),
            )
            .child(
                div()
                    .ml_auto()
                    .flex()
                    .items_center()
                    .gap(px(5.))
                    .px(px(8.))
                    .py(px(3.))
                    .rounded(theme.metrics.small_radius)
                    .border_1()
                    .border_color(theme.colors.line)
                    .bg(theme.colors.background)
                    .text_color(theme.colors.muted)
                    .child(Icon::new(IconName::Search).color(theme.colors.muted))
                    .child(DesignerTexts::SEARCH),
            )
    }

    fn grid(
        &self,
        theme: &Theme,
        rows: &[PropertyRow],
        cx: &mut Context<DesignerWorkspace>,
    ) -> impl IntoElement {
        let mut grid = div()
            .id("property-grid")
            .flex()
            .flex_col()
            .flex_1()
            .min_h_0()
            .overflow_y_scroll();
        let mut current_category = String::new();
        for (index, row) in rows.iter().enumerate() {
            if row.category != current_category {
                current_category.clone_from(&row.category);
                grid = grid.child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(5.))
                        .px(px(8.))
                        .py(px(4.))
                        .bg(theme.colors.sunken)
                        .border_b_1()
                        .border_color(theme.colors.line)
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_size(px(11.5))
                        .child(Icon::new(IconName::Down).color(theme.colors.muted))
                        .child(row.category.clone()),
                );
            }
            grid = grid.child(self.row(theme, index, row, cx));
        }
        grid
    }

    fn row(
        &self,
        theme: &Theme,
        index: usize,
        row: &PropertyRow,
        cx: &mut Context<DesignerWorkspace>,
    ) -> impl IntoElement {
        let focused = self.workspace.focused_property() == Some(&row.key) && !row.read_only;
        let editing = self
            .workspace
            .property_editor()
            .filter(|editor| editor.key == row.key);
        let value: AnyElement = match editing.map(|editor| &editor.field) {
            Some(EditorField::Text(field)) => field.clone().into_any_element(),
            Some(EditorField::Choice(field)) => field.clone().into_any_element(),
            None => {
                let color = if row.is_event && row.modified {
                    theme.colors.accent_ink
                } else if row.is_event || row.read_only {
                    theme.colors.muted
                } else {
                    theme.colors.ink
                };
                div()
                    .truncate()
                    .text_color(color)
                    .when(row.modified && !row.is_event, |value| {
                        value.font_weight(FontWeight::SEMIBOLD)
                    })
                    .when(row.is_event, |value| {
                        value
                            .font_family(theme.fonts.mono.clone())
                            .text_size(px(11.))
                    })
                    .child(row.value.clone())
                    .into_any_element()
            }
        };
        let key = row.key.clone();
        let read_only = row.read_only;
        let name_color = if focused {
            theme.colors.selection
        } else {
            theme.colors.muted
        };
        div()
            .id(("property", index))
            .flex()
            .items_center()
            .min_h(px(25.))
            .border_b_1()
            .border_color(theme.colors.line)
            .when(focused, |line| line.bg(theme.colors.selection_soft))
            .on_click(cx.listener(move |workspace, _, window, cx| {
                if read_only {
                    workspace.focus_property(key.clone(), cx);
                } else {
                    workspace.begin_edit(&key, window, cx);
                }
            }))
            .child(
                div()
                    .flex_none()
                    .w(self.workspace.settings().property_name_width)
                    .h_full()
                    .flex()
                    .items_center()
                    .pl(px(22.))
                    .pr(px(8.))
                    .border_r_1()
                    .border_color(theme.colors.line)
                    .text_color(name_color)
                    .child(div().truncate().child(row.label.clone())),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .flex_1()
                    .min_w_0()
                    .px(px(8.))
                    .child(value),
            )
    }

    fn description(&self, theme: &Theme, rows: &[PropertyRow]) -> impl IntoElement {
        let focused = self.workspace.focused_property();
        let mut title = String::new();
        let mut text = String::new();
        for row in rows {
            if Some(&row.key) == focused {
                title.clone_from(&row.label);
                text.clone_from(&row.description);
            }
        }
        if title.is_empty()
            && let Some(first) = rows.first()
        {
            title.clone_from(&first.label);
            text.clone_from(&first.description);
        }
        div()
            .flex()
            .flex_col()
            .flex_none()
            .gap(px(3.))
            .min_h(self.workspace.settings().description_height)
            .px(px(10.))
            .pt(px(8.))
            .pb(px(10.))
            .bg(theme.colors.chrome)
            .border_t_1()
            .border_color(theme.colors.line)
            .child(div().font_weight(FontWeight::SEMIBOLD).child(title))
            .child(
                div()
                    .text_color(theme.colors.muted)
                    .text_size(px(11.5))
                    .child(text),
            )
    }
}
