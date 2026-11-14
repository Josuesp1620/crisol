use crisol_ui::icon::Icon;
use crisol_ui::theme::Theme;
use gpui::{
    App, FontWeight, IntoElement, ParentElement, RenderOnce, Styled, Window, div,
    prelude::FluentBuilder, px,
};

use super::Segment;

/// A row of exclusive options sharing one border, like the device profiles.
#[derive(IntoElement)]
pub struct SegmentedControl {
    segments: Vec<Segment>,
}

impl SegmentedControl {
    pub fn new(segments: Vec<Segment>) -> Self {
        Self { segments }
    }
}

impl RenderOnce for SegmentedControl {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx);
        let count = self.segments.len();
        let mut control = div()
            .flex()
            .flex_none()
            .rounded(theme.metrics.small_radius)
            .border_1()
            .border_color(theme.colors.line)
            .overflow_hidden();
        for (position, segment) in self.segments.into_iter().enumerate() {
            let color = if segment.active {
                theme.colors.selection
            } else {
                theme.colors.muted
            };
            control = control.child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(5.))
                    .px(px(9.))
                    .py(px(4.))
                    .text_color(color)
                    .when(segment.active, |item| {
                        item.bg(theme.colors.selection_soft)
                            .font_weight(FontWeight::SEMIBOLD)
                    })
                    .when(!segment.available, |item| item.opacity(0.45))
                    .when(position + 1 < count, |item| {
                        item.border_r_1().border_color(theme.colors.line)
                    })
                    .child(Icon::new(segment.icon).color(color))
                    .child(segment.label),
            );
        }
        control
    }
}
