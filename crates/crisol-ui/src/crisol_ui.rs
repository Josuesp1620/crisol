use gpui::App;

use crate::assets::FontLoader;
use crate::components::text_box::TextBoxKeyBindings;
use crate::theme::{Theme, ThemeMode};

/// Prepares an application to use the components: theme, fonts and key bindings.
pub struct CrisolUi;

impl CrisolUi {
    pub fn install(mode: ThemeMode, cx: &mut App) -> gpui::Result<()> {
        cx.set_global(Theme::for_mode(mode));
        FontLoader::load(cx)?;
        TextBoxKeyBindings::install(cx);
        Ok(())
    }
}
