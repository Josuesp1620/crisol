use gpui::{App, Global};

use super::{ThemeColors, ThemeFonts, ThemeMetrics, ThemeMode};

/// The active theme, stored once per application and read by every component.
#[derive(Debug, Clone, PartialEq)]
pub struct Theme {
    pub mode: ThemeMode,
    pub colors: ThemeColors,
    pub fonts: ThemeFonts,
    pub metrics: ThemeMetrics,
}

impl Global for Theme {}

impl Theme {
    pub fn for_mode(mode: ThemeMode) -> Self {
        let colors = match mode {
            ThemeMode::Light => ThemeColors::light(),
            ThemeMode::Dark => ThemeColors::dark(),
        };
        Self {
            mode,
            colors,
            fonts: ThemeFonts::default(),
            metrics: ThemeMetrics::default(),
        }
    }

    pub fn of(cx: &App) -> &Theme {
        cx.global::<Theme>()
    }

    pub fn set_mode(mode: ThemeMode, cx: &mut App) {
        cx.set_global(Theme::for_mode(mode));
        cx.refresh_windows();
    }
}
