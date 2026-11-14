use crisol_ui::icon::{Icon, IconName};
use crisol_ui::theme::Theme;
use gpui::{AnyElement, Context, IntoElement, ParentElement, Styled, Window, div, px};

use crate::chrome::{PaneHeader, TreeRow};
use crate::designer_texts::DesignerTexts;
use crate::workspace::DesignerWorkspace;

/// The project explorer: the forms and Rust files next to the open form.
pub struct ExplorerPanel<'workspace> {
    workspace: &'workspace DesignerWorkspace,
}

impl<'workspace> ExplorerPanel<'workspace> {
    pub fn new(workspace: &'workspace DesignerWorkspace) -> Self {
        Self { workspace }
    }

    pub fn render(self, _window: &mut Window, cx: &mut Context<DesignerWorkspace>) -> AnyElement {
        let theme = Theme::of(cx).clone();
        let session = self.workspace.session();
        let mut rows = div().flex().flex_col().pt(px(4.)).overflow_hidden();
        if self.workspace.explorer().is_empty() {
            rows = rows.child(
                TreeRow::new("unsaved", 0, IconName::Form, session.file_name()).selected(true),
            );
        }
        for (index, entry) in self.workspace.explorer().iter().enumerate() {
            let current = session.path() == Some(entry.path.as_path());
            let mut row = TreeRow::new(
                ("explorer", index),
                entry.depth,
                entry.icon,
                entry.label.clone(),
            )
            .selected(current)
            .dimmed(entry.generated);
            if entry.depth == 0 {
                row = row.expanded(true);
            }
            if entry.generated {
                row = row.trailing(
                    Icon::new(IconName::Lock)
                        .size(px(12.))
                        .color(theme.colors.muted),
                );
            } else if current && session.has_unsaved_changes() {
                row = row.trailing(
                    div()
                        .font_family(theme.fonts.mono.clone())
                        .text_size(px(10.))
                        .text_color(theme.colors.accent_ink)
                        .child("M"),
                );
            }
            if entry.is_form {
                let path = entry.path.clone();
                row = row.on_click(Box::new(cx.listener(move |workspace, _, _, cx| {
                    workspace.open_path(path.clone(), cx);
                })));
            }
            rows = rows.child(row);
        }
        div()
            .flex()
            .flex_col()
            .flex_none()
            .h(self.workspace.settings().explorer_height)
            .border_b_1()
            .border_color(theme.colors.line)
            .child(PaneHeader::new(IconName::Folder, DesignerTexts::EXPLORER))
            .child(rows)
            .into_any_element()
    }
}
