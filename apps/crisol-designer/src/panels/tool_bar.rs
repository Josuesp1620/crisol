use crisol_catalog::validation::DiagnosticSeverity;
use crisol_ui::icon::{Icon, IconName};
use crisol_ui::theme::Theme;
use gpui::{
    AnyElement, Context, FontWeight, InteractiveElement, IntoElement, ParentElement,
    StatefulInteractiveElement, Styled, Window, div, px,
};

use crate::chrome::{IconButton, KeyboardHint, Segment, SegmentedControl, ToolBarSeparator};
use crate::designer_texts::DesignerTexts;
use crate::workspace::DesignerWorkspace;

/// The tool bar: file and history buttons, alignment, device profile, validity and Run.
pub struct ToolBar<'workspace> {
    workspace: &'workspace DesignerWorkspace,
}

impl<'workspace> ToolBar<'workspace> {
    pub fn new(workspace: &'workspace DesignerWorkspace) -> Self {
        Self { workspace }
    }

    pub fn render(self, _window: &mut Window, cx: &mut Context<DesignerWorkspace>) -> AnyElement {
        let theme = Theme::of(cx).clone();
        div()
            .flex()
            .flex_none()
            .items_center()
            .gap(px(2.))
            .h(self.workspace.settings().tool_bar_height)
            .px(px(10.))
            .border_b_1()
            .border_color(theme.colors.line)
            .bg(theme.colors.surface)
            .children(self.file_and_history(cx))
            .child(ToolBarSeparator)
            .children(self.layout_tools(&theme))
            .child(ToolBarSeparator)
            .child(Self::label(&theme, DesignerTexts::PROFILE))
            .child(Self::profiles())
            .child(Self::zoom(&theme))
            .child(div().flex_1())
            .child(self.validity(&theme))
            .child(Self::run_button(&theme, cx))
            .into_any_element()
    }

    fn file_and_history(&self, cx: &mut Context<DesignerWorkspace>) -> Vec<AnyElement> {
        let history = self.workspace.session().history();
        vec![
            IconButton::new("save", IconName::Save)
                .on_click(Box::new(cx.listener(|workspace, _, window, cx| {
                    workspace.save_form(window, cx);
                })))
                .into_any_element(),
            IconButton::new("undo", IconName::Undo)
                .dimmed(!history.can_undo())
                .on_click(Box::new(
                    cx.listener(|workspace, _, _, cx| workspace.undo(cx)),
                ))
                .into_any_element(),
            IconButton::new("redo", IconName::Redo)
                .dimmed(!history.can_redo())
                .on_click(Box::new(
                    cx.listener(|workspace, _, _, cx| workspace.redo(cx)),
                ))
                .into_any_element(),
        ]
    }

    /// Alignment and sizing act on several controls, so they are dimmed for one.
    fn layout_tools(&self, theme: &Theme) -> Vec<AnyElement> {
        let several = self.workspace.session().selection().nodes().len() > 1;
        let mut tools = vec![Self::label(theme, DesignerTexts::ALIGN).into_any_element()];
        for (id, icon) in [
            ("align-left", IconName::AlignLeft),
            ("align-center", IconName::AlignCenter),
            ("align-right", IconName::AlignRight),
            ("align-top", IconName::AlignTop),
            ("align-bottom", IconName::AlignBottom),
        ] {
            tools.push(
                IconButton::new(id, icon)
                    .dimmed(!several)
                    .into_any_element(),
            );
        }
        tools.push(ToolBarSeparator.into_any_element());
        tools.push(
            IconButton::new("same-size", IconName::Same)
                .dimmed(!several)
                .into_any_element(),
        );
        tools.push(
            IconButton::new("spacing", IconName::Hspace)
                .dimmed(!several)
                .into_any_element(),
        );
        tools.push(IconButton::new("tab-order", IconName::Taborder).into_any_element());
        tools
    }

    fn profiles() -> impl IntoElement {
        SegmentedControl::new(vec![
            Segment {
                icon: IconName::Monitor,
                label: DesignerTexts::PROFILE_DESKTOP.into(),
                active: true,
            },
            Segment {
                icon: IconName::Globe,
                label: DesignerTexts::PROFILE_WEB.into(),
                active: false,
            },
            Segment {
                icon: IconName::Phone,
                label: DesignerTexts::PROFILE_PHONE.into(),
                active: false,
            },
            Segment {
                icon: IconName::Templ,
                label: DesignerTexts::PROFILE_ALL.into(),
                active: false,
            },
        ])
    }

    fn zoom(theme: &Theme) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .gap(px(4.))
            .ml(px(10.))
            .px(px(8.))
            .py(px(4.))
            .rounded(theme.metrics.control_radius)
            .border_1()
            .border_color(theme.colors.line)
            .child(DesignerTexts::ZOOM)
            .child(Icon::new(IconName::Down).color(theme.colors.muted))
    }

    fn run_button(theme: &Theme, cx: &mut Context<DesignerWorkspace>) -> impl IntoElement {
        div()
            .id("run")
            .flex()
            .items_center()
            .gap(px(6.))
            .pl(px(12.))
            .pr(px(10.))
            .py(px(5.))
            .rounded(theme.metrics.control_radius)
            .bg(theme.colors.accent)
            .text_color(theme.colors.on_accent)
            .font_weight(FontWeight::SEMIBOLD)
            .cursor_pointer()
            .on_click(cx.listener(|workspace, _, _, cx| workspace.run_preview(cx)))
            .child(Icon::new(IconName::Play).color(theme.colors.on_accent))
            .child(DesignerTexts::RUN)
            .child(KeyboardHint::new(DesignerTexts::RUN_SHORTCUT).on_accent())
    }

    fn label(theme: &Theme, text: &'static str) -> impl IntoElement {
        div()
            .text_size(theme.metrics.small_text_size)
            .text_color(theme.colors.muted)
            .mr(px(4.))
            .child(text)
    }

    fn validity(&self, theme: &Theme) -> impl IntoElement {
        let mut errors = 0;
        for diagnostic in self.workspace.session().diagnostics() {
            if diagnostic.severity == DiagnosticSeverity::Error {
                errors += 1;
            }
        }
        let (icon, color, text) = if errors == 0 {
            (
                IconName::Check,
                theme.colors.success,
                DesignerTexts::VALID.to_owned(),
            )
        } else {
            (
                IconName::Err,
                theme.colors.danger,
                format!("{errors} {}", DesignerTexts::ERRORS),
            )
        };
        div()
            .flex()
            .items_center()
            .gap(px(5.))
            .mr(px(10.))
            .text_color(color)
            .font_weight(FontWeight::SEMIBOLD)
            .child(Icon::new(icon).color(color))
            .child(text)
    }
}
