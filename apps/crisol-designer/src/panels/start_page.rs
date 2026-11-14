use crisol_ui::icon::{Icon, IconName};
use crisol_ui::theme::Theme;
use gpui::{
    AnyElement, Context, FontWeight, InteractiveElement, IntoElement, ParentElement,
    StatefulInteractiveElement, Styled, Window, div, px,
};

use crate::designer_texts::DesignerTexts;
use crate::workspace::DesignerWorkspace;

/// What the designer shows when nothing is open: start actions and recent projects.
pub struct StartPage<'workspace> {
    workspace: &'workspace DesignerWorkspace,
}

impl<'workspace> StartPage<'workspace> {
    pub fn new(workspace: &'workspace DesignerWorkspace) -> Self {
        Self { workspace }
    }

    pub fn render(self, _window: &mut Window, cx: &mut Context<DesignerWorkspace>) -> AnyElement {
        let theme = Theme::of(cx).clone();
        div()
            .flex()
            .flex_col()
            .flex_1()
            .min_h_0()
            .gap(px(18.))
            .px(px(32.))
            .py(px(28.))
            .bg(theme.colors.background)
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(4.))
                    .child(
                        div()
                            .font_family(theme.fonts.display.clone())
                            .font_weight(FontWeight::BOLD)
                            .text_size(px(24.))
                            .child(DesignerTexts::START),
                    )
                    .child(
                        div()
                            .text_color(theme.colors.muted)
                            .child(DesignerTexts::START_SUBTITLE),
                    ),
            )
            .child(Self::actions(&theme, cx))
            .child(
                div()
                    .text_color(theme.colors.muted)
                    .text_size(theme.metrics.small_text_size)
                    .child(DesignerTexts::RECENT),
            )
            .child(self.recent(&theme, cx))
            .into_any_element()
    }

    fn actions(theme: &Theme, cx: &mut Context<DesignerWorkspace>) -> impl IntoElement {
        div()
            .flex()
            .gap(px(10.))
            .child(
                Self::action(
                    theme,
                    "start-new",
                    IconName::Plus,
                    DesignerTexts::NEW_PROJECT_ACTION,
                    true,
                )
                .on_click(
                    cx.listener(|workspace, _, window, cx| workspace.open_new_project(window, cx)),
                ),
            )
            .child(
                Self::action(
                    theme,
                    "start-open",
                    IconName::Folder,
                    DesignerTexts::OPEN_FORM_ACTION,
                    false,
                )
                .on_click(cx.listener(|workspace, _, window, cx| workspace.open_form(window, cx))),
            )
            .child(
                Self::action(
                    theme,
                    "start-sample",
                    IconName::Form,
                    DesignerTexts::OPEN_SAMPLE,
                    false,
                )
                .on_click(cx.listener(|workspace, _, _, cx| workspace.open_sample(cx))),
            )
    }

    fn recent(&self, theme: &Theme, cx: &mut Context<DesignerWorkspace>) -> impl IntoElement {
        let mut recent = div().flex().flex_col().gap(px(6.)).max_w(px(520.));
        if self.workspace.recent().forms().is_empty() {
            recent = recent.child(
                div()
                    .text_color(theme.colors.muted)
                    .child(DesignerTexts::NO_RECENT),
            );
        }
        for (index, path) in self.workspace.recent().forms().iter().enumerate() {
            let name = path
                .file_stem()
                .map(|stem| stem.to_string_lossy().into_owned())
                .unwrap_or_default();
            let folder = path
                .parent()
                .map(|folder| folder.display().to_string())
                .unwrap_or_default();
            let target = path.clone();
            recent = recent.child(
                div()
                    .id(("recent", index))
                    .flex()
                    .items_center()
                    .gap(px(10.))
                    .px(px(10.))
                    .py(px(8.))
                    .rounded(theme.metrics.control_radius)
                    .border_1()
                    .border_color(theme.colors.line)
                    .cursor_pointer()
                    .hover(|style| style.bg(theme.colors.sunken))
                    .on_click(cx.listener(move |workspace, _, _, cx| {
                        workspace.open_any(target.clone(), cx);
                    }))
                    .child(Icon::new(IconName::Box).color(theme.colors.accent))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .min_w_0()
                            .child(div().font_weight(FontWeight::SEMIBOLD).child(name))
                            .child(
                                div()
                                    .truncate()
                                    .font_family(theme.fonts.mono.clone())
                                    .text_size(px(10.5))
                                    .text_color(theme.colors.muted)
                                    .child(folder),
                            ),
                    ),
            );
        }
        recent
    }

    fn action(
        theme: &Theme,
        id: &'static str,
        icon: IconName,
        label: &'static str,
        primary: bool,
    ) -> gpui::Stateful<gpui::Div> {
        let (background, border, color) = if primary {
            (
                theme.colors.accent,
                theme.colors.accent,
                theme.colors.on_accent,
            )
        } else {
            (theme.colors.surface, theme.colors.line, theme.colors.ink)
        };
        div()
            .id(id)
            .flex()
            .items_center()
            .gap(px(8.))
            .px(px(14.))
            .py(px(8.))
            .rounded(theme.metrics.control_radius)
            .border_1()
            .border_color(border)
            .bg(background)
            .text_color(color)
            .font_weight(FontWeight::SEMIBOLD)
            .cursor_pointer()
            .child(Icon::new(icon).color(color))
            .child(label)
    }
}
