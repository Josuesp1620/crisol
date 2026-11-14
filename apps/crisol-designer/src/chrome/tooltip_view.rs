use crisol_ui::theme::Theme;
use gpui::{
    AnyView, App, AppContext, Context, IntoElement, ParentElement, Render, SharedString, Styled,
    Window, div, px,
};

/// The small label shown when the pointer rests on a button.
pub struct TooltipView {
    text: SharedString,
}

impl TooltipView {
    /// A tooltip builder for `.tooltip(...)`.
    pub fn builder(
        text: impl Into<SharedString>,
    ) -> impl Fn(&mut Window, &mut App) -> AnyView + 'static {
        let text = text.into();
        move |_, cx| cx.new(|_| TooltipView { text: text.clone() }).into()
    }
}

impl Render for TooltipView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = Theme::of(cx);
        div()
            .px(px(8.))
            .py(px(4.))
            .rounded(theme.metrics.small_radius)
            .bg(theme.colors.ink)
            .text_color(theme.colors.surface)
            .font_family(theme.fonts.body.clone())
            .text_size(theme.metrics.small_text_size)
            .child(self.text.clone())
    }
}
