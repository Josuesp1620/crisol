use crisol_ui::theme::Theme;
use gpui::{
    App, Hsla, IntoElement, ParentElement, RenderOnce, SharedString, Styled, Window, div, px,
};

/// A keyboard shortcut drawn as a key cap (`F5`, `Ctrl+Shift+P`).
#[derive(IntoElement)]
pub struct KeyboardHint {
    keys: SharedString,
    on_accent: bool,
}

impl KeyboardHint {
    pub fn new(keys: impl Into<SharedString>) -> Self {
        Self {
            keys: keys.into(),
            on_accent: false,
        }
    }

    /// For key caps drawn on the accent color, like the Run button.
    pub fn on_accent(mut self) -> Self {
        self.on_accent = true;
        self
    }
}

impl RenderOnce for KeyboardHint {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx);
        let (color, border, background): (Hsla, Hsla, Hsla) = if self.on_accent {
            let white = theme.colors.on_accent;
            (white, white.opacity(0.5), gpui::transparent_black())
        } else {
            (theme.colors.muted, theme.colors.line, theme.colors.surface)
        };
        div()
            .px(px(4.))
            .rounded(px(4.))
            .border_1()
            .border_b_2()
            .border_color(border)
            .bg(background)
            .text_color(color)
            .font_family(theme.fonts.mono.clone())
            .text_size(px(10.))
            .child(self.keys)
    }
}
