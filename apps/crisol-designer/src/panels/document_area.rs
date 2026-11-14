use crisol_model::identity::NodeId;
use crisol_ui::icon::{Icon, IconName};
use crisol_ui::theme::Theme;
use gpui::{AnyElement, Context, FontWeight, IntoElement, ParentElement, Styled, Window, div, px};

use super::{BottomPanel, CodePanel, StartPage};
use crate::canvas::DesignCanvas;
use crate::chrome::DocumentTab;
use crate::designer_texts::DesignerTexts;
use crate::workspace::DesignerWorkspace;

/// The middle column: document and code tabs, then the start page, a code view or the
/// selection path with the canvas, and the bottom panel.
pub struct DocumentArea<'workspace> {
    workspace: &'workspace DesignerWorkspace,
}

impl<'workspace> DocumentArea<'workspace> {
    pub fn new(workspace: &'workspace DesignerWorkspace) -> Self {
        Self { workspace }
    }

    pub fn render(self, window: &mut Window, cx: &mut Context<DesignerWorkspace>) -> AnyElement {
        let theme = Theme::of(cx).clone();
        let mut area = div()
            .flex()
            .flex_col()
            .flex_1()
            .min_w_0()
            .h_full()
            .child(self.tabs(&theme, cx));
        if self.workspace.showing_start() {
            area = area.child(StartPage::new(self.workspace).render(window, cx));
        } else if let Some(view) = self.workspace.active_code() {
            area = area.child(CodePanel::new(view).render(window, cx));
        } else {
            area = area
                .child(self.breadcrumb(&theme))
                .child(DesignCanvas::new(self.workspace).render(window, cx));
        }
        area.child(BottomPanel::new(self.workspace).render(window, cx))
            .into_any_element()
    }

    fn tabs(&self, theme: &Theme, cx: &mut Context<DesignerWorkspace>) -> impl IntoElement {
        let mut tabs = div()
            .flex()
            .flex_none()
            .h(self.workspace.settings().document_tabs_height)
            .bg(theme.colors.chrome)
            .border_b_1()
            .border_color(theme.colors.line)
            .overflow_hidden();
        if self.workspace.showing_start() {
            return tabs.child(
                DocumentTab::new("start", IconName::Box, DesignerTexts::START).active(true),
            );
        }
        let code_active = self.workspace.active_code_index();
        for (index, session) in self.workspace.sessions().iter().enumerate() {
            let label = format!(
                "{} {}",
                session.file_name(),
                DesignerTexts::DESIGN_TAB_SUFFIX
            );
            tabs = tabs.child(
                DocumentTab::new(("document", index), IconName::Form, label)
                    .active(code_active.is_none() && index == self.workspace.active_index())
                    .dirty(session.has_unsaved_changes())
                    .on_click(Box::new(cx.listener(move |workspace, _, _, cx| {
                        workspace.activate(index, cx);
                    })))
                    .on_close(Box::new(cx.listener(move |workspace, _, window, cx| {
                        workspace.close_tab(index, window, cx);
                    }))),
            );
        }
        for (index, view) in self.workspace.code_views().iter().enumerate() {
            let icon = if view.is_generated() {
                IconName::Lock
            } else {
                IconName::Rust
            };
            tabs = tabs.child(
                DocumentTab::new(("code", index), icon, view.file_name())
                    .active(code_active == Some(index))
                    .on_click(Box::new(cx.listener(move |workspace, _, _, cx| {
                        workspace.activate_code(index, cx);
                    })))
                    .on_close(Box::new(cx.listener(move |workspace, _, _, cx| {
                        workspace.close_code(index, cx);
                    }))),
            );
        }
        tabs
    }

    fn breadcrumb(&self, theme: &Theme) -> impl IntoElement {
        let session = self.workspace.session();
        let document = session.document();
        let mut path: Vec<NodeId> = Vec::new();
        let mut current = session.selection().primary();
        while let Some(node) = current {
            path.push(node);
            current = document.parent_of(node);
        }
        path.reverse();
        let mut crumb = div()
            .flex()
            .flex_none()
            .items_center()
            .gap(px(6.))
            .h(self.workspace.settings().breadcrumb_height)
            .px(px(12.))
            .border_b_1()
            .border_color(theme.colors.line)
            .font_family(theme.fonts.mono.clone())
            .text_size(theme.metrics.small_text_size)
            .text_color(theme.colors.muted);
        let last = path.len().saturating_sub(1);
        for (position, node) in path.iter().enumerate() {
            let name = if *node == document.root_id() {
                document.form().name.to_string()
            } else {
                document
                    .node(*node)
                    .map(|found| found.name.to_string())
                    .unwrap_or_default()
            };
            if position > 0 {
                crumb = crumb.child(
                    Icon::new(IconName::Right)
                        .size(px(11.))
                        .color(theme.colors.muted),
                );
            }
            if position == last {
                crumb = crumb.child(
                    div()
                        .text_color(theme.colors.ink)
                        .font_weight(FontWeight::MEDIUM)
                        .child(name),
                );
            } else {
                crumb = crumb.child(name);
            }
        }
        crumb
    }
}
