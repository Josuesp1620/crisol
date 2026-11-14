use std::path::PathBuf;
use std::rc::Rc;

use crisol_catalog::WidgetCatalog;
use crisol_ui::CrisolUi;
use crisol_ui::assets::CrisolAssets;
use crisol_ui::theme::{Theme, ThemeMode};
use gpui::{
    App, AppContext, Bounds, Focusable, TitlebarOptions, WindowAppearance, WindowBounds,
    WindowOptions, size,
};

use crate::actions::DesignerKeyBindings;
use crate::designer_settings::DesignerSettings;
use crate::designer_texts::DesignerTexts;
use crate::session::{DocumentFiles, DocumentSession};
use crate::workspace::DesignerWorkspace;

/// Starts Crisol: theme, fonts, shortcuts and the designer window.
pub struct DesignerApplication;

impl DesignerApplication {
    /// The customer form shown on first start, so the designer never opens empty.
    const SAMPLE_FORM: &'static str = include_str!("../../../examples/customer/customer.crisol");

    /// The sample form, as an unsaved document.
    pub fn sample_session(catalog: &WidgetCatalog) -> DocumentSession {
        let document = DocumentFiles::parse(Self::SAMPLE_FORM).expect("the sample form is valid");
        DocumentSession::new(document, None, catalog)
    }

    pub fn run() {
        gpui_platform::application()
            .with_assets(CrisolAssets)
            .run(|cx: &mut App| {
                CrisolUi::install(ThemeMode::Light, cx).expect("the bundled fonts load");
                DesignerKeyBindings::install(cx);
                let catalog = Rc::new(WidgetCatalog::standard());
                let settings = DesignerSettings::default();
                let bounds = Bounds::centered(
                    None,
                    size(settings.window_width, settings.window_height),
                    cx,
                );
                let options = WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    titlebar: Some(TitlebarOptions {
                        title: Some(DesignerTexts::APP_NAME.into()),
                        appears_transparent: true,
                        ..TitlebarOptions::default()
                    }),
                    ..WindowOptions::default()
                };
                let opened = cx.open_window(options, |window, cx| {
                    if matches!(
                        window.appearance(),
                        WindowAppearance::Dark | WindowAppearance::VibrantDark
                    ) {
                        Theme::set_mode(ThemeMode::Dark, cx);
                    }
                    let sample = Self::sample_session(&catalog);
                    let workspace = cx.new(|cx| DesignerWorkspace::new(sample, true, catalog, cx));
                    if let Some(argument) = std::env::args().nth(1) {
                        workspace.update(cx, |workspace, cx| {
                            workspace.open_any(PathBuf::from(argument), cx);
                        });
                    }
                    window.focus(&workspace.focus_handle(cx), cx);
                    workspace
                });
                opened.expect("the designer window opens");
                cx.activate(true);
            });
    }
}
