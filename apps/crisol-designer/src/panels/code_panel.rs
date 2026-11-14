use crisol_ui::icon::{Icon, IconName};
use crisol_ui::theme::Theme;
use gpui::{
    AnyElement, Context, FontWeight, HighlightStyle, InteractiveElement, IntoElement,
    ParentElement, StatefulInteractiveElement, Styled, StyledText, Window, div,
    prelude::FluentBuilder, px,
};

use crate::code::{CodeToken, CodeView, RustHighlighter};
use crate::workspace::DesignerWorkspace;

/// A code tab: the file with line numbers and Rust colors, read only, with a button to
/// open it in the user's editor.
pub struct CodePanel<'view> {
    view: &'view CodeView,
}

impl<'view> CodePanel<'view> {
    pub fn new(view: &'view CodeView) -> Self {
        Self { view }
    }

    pub fn render(self, _window: &mut Window, cx: &mut Context<DesignerWorkspace>) -> AnyElement {
        let theme = Theme::of(cx).clone();
        div()
            .flex()
            .flex_col()
            .flex_1()
            .min_h_0()
            .child(self.header(&theme, cx))
            .child(self.lines(&theme))
            .into_any_element()
    }

    fn header(&self, theme: &Theme, cx: &mut Context<DesignerWorkspace>) -> impl IntoElement {
        let path = self.view.path().to_path_buf();
        let (icon, note) = if self.view.is_generated() {
            (
                IconName::Lock,
                "Generated · read only · rewritten when the form is saved",
            )
        } else {
            (
                IconName::Rust,
                "Your code · Crisol only adds new methods, it never removes code",
            )
        };
        div()
            .flex()
            .items_center()
            .gap(px(6.))
            .px(px(12.))
            .py(px(6.))
            .border_b_1()
            .border_color(theme.colors.line)
            .bg(theme.colors.surface)
            .child(Icon::new(icon).color(theme.colors.muted))
            .child(
                div()
                    .font_weight(FontWeight::SEMIBOLD)
                    .child(self.view.file_name()),
            )
            .child(
                div()
                    .text_color(theme.colors.muted)
                    .text_size(px(11.))
                    .child(note),
            )
            .child(div().flex_1())
            .child(
                div()
                    .id("open-in-editor")
                    .flex()
                    .items_center()
                    .gap(px(6.))
                    .px(px(10.))
                    .py(px(4.))
                    .rounded(theme.metrics.control_radius)
                    .border_1()
                    .border_color(theme.colors.line)
                    .cursor_pointer()
                    .hover(|style| style.bg(theme.colors.sunken))
                    .on_click(cx.listener(move |workspace, _, _, cx| {
                        workspace.open_externally(&path, cx);
                    }))
                    .child(Icon::new(IconName::Code).color(theme.colors.ink))
                    .child("Open in editor"),
            )
    }

    fn lines(&self, theme: &Theme) -> impl IntoElement {
        let mut lines = div()
            .id("code-lines")
            .flex()
            .flex_col()
            .flex_1()
            .min_h_0()
            .py(px(6.))
            .overflow_y_scroll()
            .bg(theme.colors.code_background)
            .font_family(theme.fonts.mono.clone())
            .text_size(px(12.))
            .text_color(theme.colors.code_ink);
        for (number, line) in self.view.lines().iter().enumerate() {
            let marked = self.view.marked_line() == Some(number);
            let mut highlights = Vec::new();
            for (range, token) in RustHighlighter::highlight(line) {
                highlights.push((range, Self::style_of(token, theme)));
            }
            let text = if line.is_empty() {
                " ".to_owned()
            } else {
                line.clone()
            };
            lines = lines.child(
                div()
                    .flex()
                    .when(marked, |row| row.bg(theme.colors.success_soft))
                    .child(
                        div()
                            .w(px(48.))
                            .flex_none()
                            .pr(px(12.))
                            .flex()
                            .justify_end()
                            .text_color(theme.colors.code_dim)
                            .child((number + 1).to_string()),
                    )
                    .child(
                        div()
                            .whitespace_nowrap()
                            .child(StyledText::new(text).with_highlights(highlights)),
                    ),
            );
        }
        lines
    }

    fn style_of(token: CodeToken, theme: &Theme) -> HighlightStyle {
        let color = match token {
            CodeToken::Keyword => theme.colors.code_keyword,
            CodeToken::String => theme.colors.code_string,
            CodeToken::Comment => theme.colors.code_dim,
            CodeToken::Number | CodeToken::Type => theme.colors.selection,
        };
        HighlightStyle {
            color: Some(color),
            ..HighlightStyle::default()
        }
    }
}
