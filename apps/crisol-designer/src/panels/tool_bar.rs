use crisol_catalog::validation::DiagnosticSeverity;
use crisol_engine::arranging::{Alignment, AlignmentTool};
use crisol_ui::icon::{Icon, IconName};
use crisol_ui::theme::Theme;
use gpui::{
    AnyElement, Context, FontWeight, InteractiveElement, IntoElement, ParentElement,
    StatefulInteractiveElement, Styled, Window, div, px,
};

use crate::chrome::{
    IconButton, KeyboardHint, Segment, SegmentedControl, ToolBarSeparator, TooltipView,
};
use crate::designer_texts::DesignerTexts;
use crate::workspace::DesignerWorkspace;

/// The tool bar: file and history, alignment, device profile, validity, build and run.
pub struct ToolBar<'workspace> {
    workspace: &'workspace DesignerWorkspace,
}

impl<'workspace> ToolBar<'workspace> {
    const ALIGNMENTS: [(&'static str, IconName, Alignment); 5] = [
        ("align-left", IconName::AlignLeft, Alignment::Left),
        ("align-center", IconName::AlignCenter, Alignment::Center),
        ("align-right", IconName::AlignRight, Alignment::Right),
        ("align-top", IconName::AlignTop, Alignment::Top),
        ("align-bottom", IconName::AlignBottom, Alignment::Bottom),
    ];

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
            .children(self.layout_tools(&theme, cx))
            .child(ToolBarSeparator)
            .child(Self::label(&theme, DesignerTexts::PROFILE))
            .child(Self::profiles())
            .child(Self::zoom(&theme))
            .child(div().flex_1())
            .child(self.validity(&theme))
            .child(
                IconButton::new("build", IconName::Box)
                    .tooltip("Build project (Ctrl+Shift+B)")
                    .dimmed(self.workspace.project().is_none() || self.workspace.building())
                    .on_click(Box::new(
                        cx.listener(|workspace, _, _, cx| workspace.build_project(cx)),
                    )),
            )
            .child(
                IconButton::new("view-code", IconName::Code)
                    .tooltip("View code (F7)")
                    .on_click(Box::new(
                        cx.listener(|workspace, _, _, cx| workspace.view_code(None, cx)),
                    )),
            )
            .child(Self::run_button(&theme, cx))
            .into_any_element()
    }

    fn file_and_history(&self, cx: &mut Context<DesignerWorkspace>) -> Vec<AnyElement> {
        let history = self.workspace.session().history();
        vec![
            IconButton::new("new-project", IconName::Plus)
                .tooltip("New project (Ctrl+Shift+N)")
                .on_click(Box::new(cx.listener(|workspace, _, window, cx| {
                    workspace.open_new_project(window, cx);
                })))
                .into_any_element(),
            IconButton::new("open", IconName::Folder)
                .tooltip("Open project or form (Ctrl+O)")
                .on_click(Box::new(cx.listener(|workspace, _, window, cx| {
                    workspace.open_form(window, cx);
                })))
                .into_any_element(),
            IconButton::new("save", IconName::Save)
                .tooltip("Save (Ctrl+S)")
                .on_click(Box::new(cx.listener(|workspace, _, window, cx| {
                    workspace.save_form(window, cx);
                })))
                .into_any_element(),
            IconButton::new("undo", IconName::Undo)
                .tooltip("Undo (Ctrl+Z)")
                .dimmed(!history.can_undo())
                .on_click(Box::new(
                    cx.listener(|workspace, _, _, cx| workspace.undo(cx)),
                ))
                .into_any_element(),
            IconButton::new("redo", IconName::Redo)
                .tooltip("Redo (Ctrl+Y)")
                .dimmed(!history.can_redo())
                .on_click(Box::new(
                    cx.listener(|workspace, _, _, cx| workspace.redo(cx)),
                ))
                .into_any_element(),
        ]
    }

    /// Alignment works on two or more controls of one absolute panel.
    fn layout_tools(&self, theme: &Theme, cx: &mut Context<DesignerWorkspace>) -> Vec<AnyElement> {
        let selection = self.workspace.session().selection().nodes();
        let can_align =
            AlignmentTool::new(self.workspace.session().document()).can_align(selection);
        let mut tools = vec![Self::label(theme, DesignerTexts::ALIGN).into_any_element()];
        for (id, icon, alignment) in Self::ALIGNMENTS {
            tools.push(Self::arrange_button(id, icon, alignment, can_align, cx));
        }
        tools.push(ToolBarSeparator.into_any_element());
        tools.push(Self::arrange_button(
            "same-size",
            IconName::Same,
            Alignment::SameSize,
            can_align,
            cx,
        ));
        tools.push(Self::arrange_button(
            "spacing",
            IconName::Hspace,
            Alignment::SpaceHorizontally,
            can_align,
            cx,
        ));
        tools.push(
            IconButton::new("tab-order", IconName::Taborder)
                .dimmed(true)
                .tooltip(DesignerTexts::TAB_ORDER_LATER)
                .into_any_element(),
        );
        tools
    }

    fn arrange_button(
        id: &'static str,
        icon: IconName,
        alignment: Alignment,
        enabled: bool,
        cx: &mut Context<DesignerWorkspace>,
    ) -> AnyElement {
        let tooltip = if enabled {
            alignment.label()
        } else {
            DesignerTexts::ALIGN_HINT
        };
        div()
            .id(id)
            .tooltip(TooltipView::builder(tooltip))
            .child(
                IconButton::new(id, icon)
                    .dimmed(!enabled)
                    .on_click(Box::new(cx.listener(move |workspace, _, _, cx| {
                        workspace.arrange_selection(alignment, cx);
                    }))),
            )
            .into_any_element()
    }

    fn profiles() -> impl IntoElement {
        div()
            .id("profiles")
            .tooltip(TooltipView::builder(
                "Web, phone and side-by-side previews come in phases 6 and 7",
            ))
            .child(SegmentedControl::new(vec![
                Segment {
                    icon: IconName::Monitor,
                    label: DesignerTexts::PROFILE_DESKTOP.into(),
                    active: true,
                    available: true,
                },
                Segment {
                    icon: IconName::Globe,
                    label: DesignerTexts::PROFILE_WEB.into(),
                    active: false,
                    available: false,
                },
                Segment {
                    icon: IconName::Phone,
                    label: DesignerTexts::PROFILE_PHONE.into(),
                    active: false,
                    available: false,
                },
                Segment {
                    icon: IconName::Templ,
                    label: DesignerTexts::PROFILE_ALL.into(),
                    active: false,
                    available: false,
                },
            ]))
    }

    fn zoom(theme: &Theme) -> impl IntoElement {
        div()
            .id("zoom")
            .tooltip(TooltipView::builder(DesignerTexts::ZOOM_LATER))
            .flex()
            .items_center()
            .gap(px(4.))
            .ml(px(10.))
            .px(px(8.))
            .py(px(4.))
            .rounded(theme.metrics.control_radius)
            .border_1()
            .border_color(theme.colors.line)
            .opacity(0.6)
            .child(DesignerTexts::ZOOM)
            .child(Icon::new(IconName::Down).color(theme.colors.muted))
    }

    fn run_button(theme: &Theme, cx: &mut Context<DesignerWorkspace>) -> impl IntoElement {
        let hover = theme.colors.accent_ink;
        div()
            .id("run")
            .tooltip(TooltipView::builder(
                "Preview the form (F5) · Run the project (Ctrl+F5)",
            ))
            .flex()
            .items_center()
            .gap(px(6.))
            .ml(px(6.))
            .pl(px(12.))
            .pr(px(10.))
            .py(px(5.))
            .rounded(theme.metrics.control_radius)
            .bg(theme.colors.accent)
            .hover(move |style| style.bg(hover))
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
