use crisol_catalog::validation::DiagnosticSeverity;
use crisol_ui::icon::{Icon, IconName};
use crisol_ui::theme::Theme;
use gpui::{
    AnyElement, Context, Hsla, InteractiveElement, IntoElement, ParentElement,
    StatefulInteractiveElement, Styled, Window, div, prelude::FluentBuilder, px,
};

use crate::designer_texts::DesignerTexts;
use crate::session::OutputKind;
use crate::workspace::{BottomTab, DesignerWorkspace};

/// The panel under the canvas: error list, output and history.
pub struct BottomPanel<'workspace> {
    workspace: &'workspace DesignerWorkspace,
}

impl<'workspace> BottomPanel<'workspace> {
    pub fn new(workspace: &'workspace DesignerWorkspace) -> Self {
        Self { workspace }
    }

    pub fn render(self, _window: &mut Window, cx: &mut Context<DesignerWorkspace>) -> AnyElement {
        let theme = Theme::of(cx).clone();
        let active = self.workspace.bottom_tab();
        let mut tabs = div()
            .flex()
            .flex_none()
            .h(px(30.))
            .bg(theme.colors.chrome)
            .border_b_1()
            .border_color(theme.colors.line);
        for (tab, icon, label) in [
            (BottomTab::Errors, IconName::Err, DesignerTexts::ERROR_LIST),
            (BottomTab::Output, IconName::Term, DesignerTexts::OUTPUT),
            (
                BottomTab::History,
                IconName::History,
                DesignerTexts::HISTORY,
            ),
        ] {
            let selected = tab == active;
            let color = if selected {
                theme.colors.ink
            } else {
                theme.colors.muted
            };
            tabs = tabs.child(
                div()
                    .id(label)
                    .relative()
                    .flex()
                    .items_center()
                    .gap(px(6.))
                    .px(px(12.))
                    .text_color(color)
                    .cursor_pointer()
                    .when(selected, |item| {
                        item.bg(theme.colors.surface).child(
                            div()
                                .absolute()
                                .bottom_0()
                                .left_0()
                                .right_0()
                                .h(px(2.))
                                .bg(theme.colors.accent),
                        )
                    })
                    .on_click(
                        cx.listener(move |workspace, _, _, cx| workspace.show_bottom_tab(tab, cx)),
                    )
                    .child(Icon::new(icon).color(color))
                    .child(label),
            );
        }
        tabs = tabs.child(div().flex_1()).child(
            div()
                .flex()
                .items_center()
                .px(px(10.))
                .child(Icon::new(IconName::Min).color(theme.colors.muted)),
        );
        let content = match active {
            BottomTab::Errors => self.errors(&theme, cx),
            BottomTab::Output => self.output(&theme),
            BottomTab::History => self.history(&theme),
        };
        div()
            .flex()
            .flex_col()
            .flex_none()
            .h(self.workspace.settings().bottom_panel_height)
            .border_t_1()
            .border_color(theme.colors.line)
            .bg(theme.colors.surface)
            .child(tabs)
            .child(
                div()
                    .id("bottom-content")
                    .flex_1()
                    .min_h_0()
                    .overflow_y_scroll()
                    .child(content),
            )
            .into_any_element()
    }

    fn errors(&self, theme: &Theme, cx: &mut Context<DesignerWorkspace>) -> AnyElement {
        let diagnostics = self.workspace.session().diagnostics();
        let mut counts = [0; 3];
        for diagnostic in diagnostics {
            counts[diagnostic.severity as usize] += 1;
        }
        let summary = div()
            .flex()
            .gap(px(14.))
            .px(px(12.))
            .py(px(5.))
            .border_b_1()
            .border_color(theme.colors.line)
            .child(Self::count(
                IconName::Err,
                theme.colors.danger,
                counts[0],
                DesignerTexts::ERRORS,
            ))
            .child(Self::count(
                IconName::Warn,
                theme.colors.warning,
                counts[1],
                DesignerTexts::WARNINGS,
            ))
            .child(Self::count(
                IconName::Info,
                theme.colors.selection,
                counts[2],
                DesignerTexts::MESSAGES,
            ));
        let mut list = div().flex().flex_col().child(summary);
        for (index, diagnostic) in diagnostics.iter().enumerate() {
            let node = diagnostic.node;
            let (icon, color) = match diagnostic.severity {
                DiagnosticSeverity::Error => (IconName::Err, theme.colors.danger),
                DiagnosticSeverity::Warning => (IconName::Warn, theme.colors.warning),
                DiagnosticSeverity::Message => (IconName::Info, theme.colors.selection),
            };
            list = list.child(
                div()
                    .id(("diagnostic", index))
                    .flex()
                    .items_center()
                    .gap(px(8.))
                    .px(px(12.))
                    .py(px(4.))
                    .border_b_1()
                    .border_color(theme.colors.line)
                    .cursor_pointer()
                    .hover(|style| style.bg(theme.colors.sunken))
                    .on_click(cx.listener(move |workspace, _, _, cx| workspace.reveal(node, cx)))
                    .child(Icon::new(icon).color(color))
                    .child(div().flex_1().truncate().child(diagnostic.message.clone()))
                    .child(Self::location(theme, diagnostic.location.clone(), px(190.)))
                    .child(Self::location(theme, diagnostic.topic.to_owned(), px(80.))),
            );
        }
        list.into_any_element()
    }

    fn output(&self, theme: &Theme) -> AnyElement {
        let mut list = div()
            .flex()
            .flex_col()
            .font_family(theme.fonts.mono.clone())
            .text_size(theme.metrics.small_text_size);
        for entry in self.workspace.output().entries().iter().rev() {
            let color = match entry.kind {
                OutputKind::Info => theme.colors.ink,
                OutputKind::Success => theme.colors.success,
                OutputKind::Failure => theme.colors.danger,
            };
            list = list.child(
                div()
                    .flex()
                    .gap(px(12.))
                    .px(px(12.))
                    .py(px(4.))
                    .child(
                        div()
                            .text_color(theme.colors.muted)
                            .child(entry.time.clone()),
                    )
                    .child(div().text_color(color).child(entry.text.clone())),
            );
        }
        list.into_any_element()
    }

    fn history(&self, theme: &Theme) -> AnyElement {
        let mut list = div().flex().flex_col();
        for (position, entry) in self
            .workspace
            .session()
            .history()
            .recent_entries()
            .enumerate()
        {
            let current = position == 0;
            let color = if current {
                theme.colors.selection
            } else {
                theme.colors.muted
            };
            list = list.child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(8.))
                    .px(px(12.))
                    .py(px(5.))
                    .border_b_1()
                    .border_color(theme.colors.line)
                    .when(current, |row| row.bg(theme.colors.selection_soft))
                    .child(Icon::new(IconName::History).color(color))
                    .child(entry.label.clone()),
            );
        }
        if !self.workspace.session().history().can_undo() {
            list = list.child(
                div()
                    .px(px(12.))
                    .py(px(6.))
                    .text_color(theme.colors.muted)
                    .child(DesignerTexts::NOTHING_TO_SHOW),
            );
        }
        list.into_any_element()
    }

    fn count(icon: IconName, color: Hsla, count: usize, label: &'static str) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .gap(px(5.))
            .child(Icon::new(icon).color(color))
            .child(format!("{count} {label}"))
    }

    fn location(theme: &Theme, text: String, width: gpui::Pixels) -> impl IntoElement {
        div()
            .w(width)
            .flex_none()
            .truncate()
            .font_family(theme.fonts.mono.clone())
            .text_size(px(10.5))
            .text_color(theme.colors.muted)
            .child(text)
    }
}
