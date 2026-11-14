use gpui::{App, Hsla, IntoElement, Pixels, RenderOnce, Styled, Window, svg};

use super::IconName;
use crate::theme::Theme;

/// One icon, tinted with a theme color.
#[derive(IntoElement)]
pub struct Icon {
    name: IconName,
    size: Option<Pixels>,
    color: Option<Hsla>,
}

impl Icon {
    pub fn new(name: IconName) -> Self {
        Self {
            name,
            size: None,
            color: None,
        }
    }

    pub fn size(mut self, size: Pixels) -> Self {
        self.size = Some(size);
        self
    }

    pub fn color(mut self, color: Hsla) -> Self {
        self.color = Some(color);
        self
    }
}

impl RenderOnce for Icon {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx);
        let size = self.size.unwrap_or(theme.metrics.icon_size);
        let color = self.color.unwrap_or(theme.colors.ink);
        svg()
            .path(self.name.path())
            .size(size)
            .flex_none()
            .text_color(color)
    }
}
