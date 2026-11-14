use std::rc::Rc;

use crisol_catalog::WidgetCatalog;
use crisol_model::document::FormDocument;
use crisol_render::{DesignRenderer, LayoutRecorder, LiveControls};
use crisol_ui::theme::Theme;
use gpui::{Context, IntoElement, ParentElement, Render, Styled, Window, div};

/// A form running with working controls, opened by Run.
pub struct PreviewForm {
    document: FormDocument,
    catalog: Rc<WidgetCatalog>,
    controls: LiveControls,
}

impl PreviewForm {
    pub fn new(document: FormDocument, catalog: Rc<WidgetCatalog>, cx: &mut Context<Self>) -> Self {
        let controls = LiveControls::build(&document, &catalog, cx);
        Self {
            document,
            catalog,
            controls,
        }
    }
}

impl Render for PreviewForm {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = Theme::of(cx);
        let body = DesignRenderer::new(
            &self.document,
            self.catalog.clone(),
            LayoutRecorder::default(),
        )
        .with_live_controls(&self.controls)
        .render();
        div()
            .size_full()
            .flex()
            .flex_col()
            .bg(theme.colors.surface)
            .text_color(theme.colors.ink)
            .font_family(theme.fonts.body.clone())
            .text_size(theme.metrics.text_size)
            .child(body)
    }
}
