use crisol_ui::icon::{Icon, IconName};
use crisol_ui::theme::Theme;
use gpui::{
    AnyElement, Context, FontWeight, InteractiveElement, IntoElement, ParentElement,
    StatefulInteractiveElement, Styled, Window, WindowControlArea, div, prelude::FluentBuilder, px,
};

use super::MenuDropdown;
use crate::chrome::KeyboardHint;
use crate::designer_texts::DesignerTexts;
use crate::workspace::DesignerWorkspace;

/// The window's title bar: brand, menus, command search, project and window buttons.
pub struct TitleBar<'workspace> {
    workspace: &'workspace DesignerWorkspace,
}

impl<'workspace> TitleBar<'workspace> {
    pub fn new(workspace: &'workspace DesignerWorkspace) -> Self {
        Self { workspace }
    }

    pub fn render(self, window: &mut Window, cx: &mut Context<DesignerWorkspace>) -> AnyElement {
        let theme = Theme::of(cx).clone();
        let mut menus = div().flex().items_center();
        for (index, title) in DesignerTexts::MENUS.iter().enumerate() {
            let open = self.workspace.open_menu() == Some(index);
            let mut item = div()
                .id(("menu", index))
                .relative()
                .px(px(9.))
                .py(px(5.))
                .rounded(theme.metrics.small_radius)
                .cursor_pointer()
                .when(open, |item| {
                    item.bg(theme.colors.surface)
                        .border_1()
                        .border_color(theme.colors.line)
                })
                .on_click(cx.listener(move |workspace, _, _, cx| workspace.toggle_menu(index, cx)))
                .on_hover(cx.listener(move |workspace, hovered: &bool, _, cx| {
                    if *hovered {
                        workspace.hover_menu(index, cx);
                    }
                }))
                .child(*title);
            if open {
                item = item.child(MenuDropdown::new(self.workspace, index).render(window, cx));
            }
            menus = menus.child(item);
        }
        let project = self
            .workspace
            .session()
            .path()
            .and_then(|path| path.parent())
            .and_then(|folder| folder.file_name())
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_default();
        div()
            .id("title-bar")
            .flex()
            .flex_none()
            .items_center()
            .gap(px(14.))
            .h(self.workspace.settings().title_bar_height)
            .pl(px(12.))
            .bg(theme.colors.chrome)
            .border_b_1()
            .border_color(theme.colors.line)
            .window_control_area(WindowControlArea::Drag)
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(6.))
                    .font_family(theme.fonts.display.clone())
                    .font_weight(FontWeight::BOLD)
                    .text_size(px(13.))
                    .child(
                        Icon::new(IconName::Box)
                            .size(px(18.))
                            .color(theme.colors.accent),
                    )
                    .child(DesignerTexts::APP_NAME),
            )
            .child(menus)
            .child(
                div()
                    .ml_auto()
                    .w(px(330.))
                    .flex()
                    .items_center()
                    .gap(px(8.))
                    .px(px(8.))
                    .py(px(4.))
                    .rounded(theme.metrics.control_radius)
                    .border_1()
                    .border_color(theme.colors.line)
                    .bg(theme.colors.surface)
                    .text_color(theme.colors.muted)
                    .child(Icon::new(IconName::Search).color(theme.colors.muted))
                    .child(div().flex_1().child(DesignerTexts::COMMAND_SEARCH))
                    .child(KeyboardHint::new(DesignerTexts::COMMAND_SEARCH_SHORTCUT)),
            )
            .child(
                div()
                    .ml_auto()
                    .font_family(theme.fonts.mono.clone())
                    .text_size(theme.metrics.small_text_size)
                    .text_color(theme.colors.muted)
                    .child(project),
            )
            .child(Self::window_buttons(&theme))
            .into_any_element()
    }

    fn window_buttons(theme: &Theme) -> impl IntoElement {
        let mut buttons = div().flex().h_full();
        for (icon, area) in [
            (IconName::Min, WindowControlArea::Min),
            (IconName::Max, WindowControlArea::Max),
            (IconName::X, WindowControlArea::Close),
        ] {
            buttons = buttons.child(
                div()
                    .id(("window-button", icon as usize))
                    .flex()
                    .items_center()
                    .justify_center()
                    .w(px(44.))
                    .h_full()
                    .window_control_area(area)
                    .hover(|style| style.bg(theme.colors.sunken))
                    .child(Icon::new(icon).color(theme.colors.muted)),
            );
        }
        buttons
    }
}
