use crisol_catalog::schema::{ToolboxGroup, WidgetSchema};
use crisol_ui::icon::{Icon, IconName};
use crisol_ui::theme::Theme;
use gpui::{
    AnyElement, AppContext, ClickEvent, Context, FontWeight, InteractiveElement, IntoElement,
    ParentElement, StatefulInteractiveElement, Styled, Window, div, prelude::FluentBuilder, px,
};

use crate::chrome::{PaneHeader, TooltipView};
use crate::designer_texts::DesignerTexts;
use crate::workspace::{DesignerWorkspace, ToolboxDrag, ToolboxDragPreview};

/// The toolbox: controls grouped as in Visual Studio, filtered by the search box and
/// dragged onto the canvas or added with a double-click.
pub struct ToolboxPanel<'workspace> {
    workspace: &'workspace DesignerWorkspace,
}

impl<'workspace> ToolboxPanel<'workspace> {
    pub fn new(workspace: &'workspace DesignerWorkspace) -> Self {
        Self { workspace }
    }

    pub fn render(self, _window: &mut Window, cx: &mut Context<DesignerWorkspace>) -> AnyElement {
        let theme = Theme::of(cx).clone();
        let query = self
            .workspace
            .toolbox_search()
            .read(cx)
            .value()
            .to_lowercase();
        let mut list = div()
            .id("toolbox-list")
            .flex()
            .flex_col()
            .flex_1()
            .min_h_0()
            .overflow_y_scroll()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .m(px(8.))
                    .child(self.workspace.toolbox_search().clone()),
            )
            .child(Self::item_row(
                &theme,
                IconName::Pointer,
                DesignerTexts::POINTER,
                true,
                None,
            ));
        for group in ToolboxGroup::VISIBLE {
            let label = group.label();
            let collapsed = self.workspace.is_group_collapsed(label) && query.is_empty();
            list = list.child(
                div()
                    .id(label)
                    .cursor_pointer()
                    .on_click(
                        cx.listener(move |workspace, _, _, cx| workspace.toggle_group(label, cx)),
                    )
                    .child(Self::group_header(&theme, label, !collapsed, None)),
            );
            if collapsed {
                continue;
            }
            for schema in self.workspace.catalog().in_group(group) {
                if !query.is_empty() && !schema.label.to_lowercase().contains(&query) {
                    continue;
                }
                list = list.child(Self::item(&theme, schema, cx));
            }
        }
        for later in DesignerTexts::LATER_GROUPS {
            list = list.child(
                div()
                    .id(later)
                    .tooltip(TooltipView::builder(DesignerTexts::COMING_LATER))
                    .child(Self::group_header(
                        &theme,
                        later,
                        false,
                        Some(DesignerTexts::LATER_PHASE),
                    )),
            );
        }
        list = list.child(
            div()
                .mt_auto()
                .m(px(10.))
                .p(px(8.))
                .rounded(theme.metrics.control_radius)
                .bg(theme.colors.sunken)
                .text_color(theme.colors.muted)
                .text_size(theme.metrics.small_text_size)
                .child(DesignerTexts::TOOLBOX_HINT),
        );
        div()
            .flex()
            .flex_col()
            .flex_none()
            .w(self.workspace.settings().toolbox_width)
            .h_full()
            .bg(theme.colors.surface)
            .border_r_1()
            .border_color(theme.colors.line)
            .child(PaneHeader::new(IconName::Templ, DesignerTexts::TOOLBOX))
            .child(list)
            .into_any_element()
    }

    fn item(
        theme: &Theme,
        schema: &WidgetSchema,
        cx: &mut Context<DesignerWorkspace>,
    ) -> impl IntoElement {
        let tag = if schema.desktop_only {
            Some(DesignerTexts::DESKTOP_ONLY)
        } else {
            None
        };
        let icon = Self::icon_for(schema.icon);
        let drag = ToolboxDrag {
            kind: schema.kind,
            label: schema.label.into(),
            icon,
        };
        let kind = schema.kind;
        div()
            .id(schema.kind)
            .tooltip(TooltipView::builder(schema.description))
            .on_drag(drag, |drag, _, _, cx| {
                cx.new(|_| ToolboxDragPreview { drag: drag.clone() })
            })
            .on_click(cx.listener(move |workspace, event: &ClickEvent, _, cx| {
                if event.click_count() >= 2 {
                    workspace.add_control_to_selection(kind, cx);
                }
            }))
            .child(Self::item_row(theme, icon, schema.label, false, tag))
    }

    fn group_header(
        theme: &Theme,
        title: &'static str,
        expanded: bool,
        tag: Option<&'static str>,
    ) -> impl IntoElement {
        let icon = if expanded {
            IconName::Down
        } else {
            IconName::Right
        };
        let color = if tag.is_none() {
            theme.colors.ink
        } else {
            theme.colors.muted
        };
        div()
            .flex()
            .items_center()
            .gap(px(5.))
            .px(px(10.))
            .pt(px(6.))
            .pb(px(3.))
            .text_size(px(11.5))
            .text_color(color)
            .when(tag.is_none(), |header| {
                header.font_weight(FontWeight::SEMIBOLD)
            })
            .child(Icon::new(icon).color(theme.colors.muted))
            .child(title)
            .when_some(tag, |header, text| header.child(Self::tag(theme, text)))
    }

    fn item_row(
        theme: &Theme,
        icon: IconName,
        label: &'static str,
        active: bool,
        tag: Option<&'static str>,
    ) -> impl IntoElement {
        let color = if active {
            theme.colors.selection
        } else {
            theme.colors.ink
        };
        div()
            .flex()
            .items_center()
            .gap(px(8.))
            .h(px(26.))
            .pl(px(24.))
            .pr(px(10.))
            .text_color(color)
            .cursor_pointer()
            .when(active, |row| row.bg(theme.colors.selection_soft))
            .when(!active, |row| {
                row.hover(|style| style.bg(theme.colors.sunken))
            })
            .child(Icon::new(icon).color(color))
            .child(label)
            .when_some(tag, |row, text| row.child(Self::tag(theme, text)))
    }

    fn tag(theme: &Theme, text: &'static str) -> impl IntoElement {
        div()
            .ml_auto()
            .font_family(theme.fonts.mono.clone())
            .text_size(px(9.5))
            .text_color(theme.colors.muted)
            .font_weight(FontWeight::NORMAL)
            .child(text)
    }

    /// The catalog names icons by role; this picks the matching icon of the set.
    pub fn icon_for(name: &str) -> IconName {
        match name {
            "label" => IconName::Label,
            "button" => IconName::Button,
            "textbox" => IconName::Textbox,
            "checkbox" => IconName::Checkbox,
            "radio" => IconName::Radio,
            "combo" => IconName::Combo,
            "stack" => IconName::Stack,
            "grid" => IconName::Grid,
            "group" => IconName::Group,
            "panel" => IconName::Panel,
            _ => IconName::Form,
        }
    }
}
