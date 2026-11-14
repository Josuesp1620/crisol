use std::rc::Rc;

use crisol_catalog::WidgetCatalog;
use crisol_model::document::FormDocument;
use gpui::{App, AppContext, Bounds, TitlebarOptions, WindowBounds, WindowOptions, px, size};

use super::preview_form::PreviewForm;
use crate::designer_texts::DesignerTexts;

/// Opens a form in its own window with working controls.
pub struct PreviewWindow;

impl PreviewWindow {
    pub fn open(
        document: FormDocument,
        catalog: Rc<WidgetCatalog>,
        cx: &mut App,
    ) -> Result<(), String> {
        let form = document.form().clone();
        let bounds = Bounds::centered(
            None,
            size(px(form.size.width as f32), px(form.size.height as f32)),
            cx,
        );
        let options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            titlebar: Some(TitlebarOptions {
                title: Some(format!("{} — {}", form.title, DesignerTexts::PREVIEW_TITLE).into()),
                ..TitlebarOptions::default()
            }),
            ..WindowOptions::default()
        };
        cx.open_window(options, |_, cx| {
            cx.new(|cx| PreviewForm::new(document, catalog, cx))
        })
        .map(|_| ())
        .map_err(|error| error.to_string())
    }
}
