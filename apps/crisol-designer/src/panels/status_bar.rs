use crisol_model::layout::{Placement, SizeMode};
use crisol_ui::icon::{Icon, IconName};
use crisol_ui::theme::Theme;
use gpui::{AnyElement, Context, IntoElement, ParentElement, Styled, Window, div, px};

use crate::designer_texts::DesignerTexts;
use crate::workspace::DesignerWorkspace;

/// The status bar: readiness, the selected control and where it sits.
pub struct StatusBar<'workspace> {
    workspace: &'workspace DesignerWorkspace,
}

impl<'workspace> StatusBar<'workspace> {
    pub fn new(workspace: &'workspace DesignerWorkspace) -> Self {
        Self { workspace }
    }

    pub fn render(self, _window: &mut Window, cx: &mut Context<DesignerWorkspace>) -> AnyElement {
        let theme = Theme::of(cx).clone();
        let mut bar = div()
            .flex()
            .flex_none()
            .items_center()
            .gap(px(18.))
            .h(self.workspace.settings().status_bar_height)
            .px(px(12.))
            .bg(theme.colors.chrome)
            .border_t_1()
            .border_color(theme.colors.line)
            .font_family(theme.fonts.mono.clone())
            .text_size(px(10.5))
            .text_color(theme.colors.muted)
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(4.))
                    .text_color(theme.colors.success)
                    .child(
                        Icon::new(IconName::Dot)
                            .size(px(10.))
                            .color(theme.colors.success),
                    )
                    .child(DesignerTexts::READY),
            );
        for part in self.selection_parts() {
            bar = bar.child(part);
        }
        bar.child(div().flex_1())
            .child(DesignerTexts::CANVAS_INFO)
            .child(format!(
                "{} · {}",
                DesignerTexts::GPUI_REVISION,
                DesignerTexts::RENDERER
            ))
            .into_any_element()
    }

    fn selection_parts(&self) -> Vec<String> {
        let session = self.workspace.session();
        let Some(node) = session.selection().primary() else {
            return Vec::new();
        };
        let Ok(found) = session.document().node(node) else {
            return Vec::new();
        };
        let mut parts = vec![found.name.to_string()];
        match &found.placement {
            Placement::Cell { cell, .. } => parts.push(format!(
                "Grid: row {}, column {}",
                cell.row + 1,
                cell.column + 1
            )),
            Placement::Absolute { rect, .. } => {
                parts.push(format!("x {} · y {}", rect.left(), rect.top()));
            }
            Placement::Flow { width, .. } => {
                parts.push(format!("Stack: width {}", Self::mode(*width)));
            }
        }
        if let Some(rect) = session.recorder().snapshot().bounds_of(node) {
            parts.push(format!("{} × {}", rect.size.width, rect.size.height));
        }
        parts
    }

    fn mode(mode: SizeMode) -> String {
        match mode {
            SizeMode::Fixed(pixels) => format!("{pixels} px"),
            SizeMode::Hug => "hug".to_owned(),
            SizeMode::Fill => "fill".to_owned(),
        }
    }
}
