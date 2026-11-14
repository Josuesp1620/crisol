use std::borrow::Cow;

use gpui::App;

/// Registers the bundled font families with GPUI's text system.
pub struct FontLoader;

impl FontLoader {
    pub fn load(cx: &App) -> gpui::Result<()> {
        let fonts: Vec<Cow<'static, [u8]>> = vec![
            Cow::Borrowed(include_bytes!(
                "../../assets/fonts/instrument-sans/InstrumentSans-Regular.ttf"
            )),
            Cow::Borrowed(include_bytes!(
                "../../assets/fonts/instrument-sans/InstrumentSans-Medium.ttf"
            )),
            Cow::Borrowed(include_bytes!(
                "../../assets/fonts/instrument-sans/InstrumentSans-SemiBold.ttf"
            )),
            Cow::Borrowed(include_bytes!(
                "../../assets/fonts/bricolage-grotesque/BricolageGrotesque-Bold.ttf"
            )),
            Cow::Borrowed(include_bytes!(
                "../../assets/fonts/jetbrains-mono/JetBrainsMono-Regular.ttf"
            )),
            Cow::Borrowed(include_bytes!(
                "../../assets/fonts/jetbrains-mono/JetBrainsMono-Medium.ttf"
            )),
        ];
        cx.text_system().add_fonts(fonts)
    }
}
