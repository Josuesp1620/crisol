use crisol_ui::prelude::*;
use gpui::{App, AppContext, Bounds, TitlebarOptions, WindowBounds, WindowOptions, px, size};

use crate::forms::customer::CustomerForm;

/// Opens the customer form in its own window, at its designed size.
pub struct CustomerWindow;

impl CustomerWindow {
    pub fn run() {
        gpui_platform::application()
            .with_assets(CrisolAssets)
            .run(|cx: &mut App| {
                CrisolUi::install(ThemeMode::Light, cx).expect("the bundled fonts load");
                let (width, height) = CustomerForm::DESIGN_SIZE;
                let bounds = Bounds::centered(None, size(px(width), px(height)), cx);
                let options = WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    titlebar: Some(TitlebarOptions {
                        title: Some(CustomerForm::TITLE.into()),
                        ..TitlebarOptions::default()
                    }),
                    ..WindowOptions::default()
                };
                cx.open_window(options, |window, cx| {
                    cx.new(|cx| CustomerForm::new(window, cx))
                })
                .expect("the window opens");
            });
    }
}
