use gpui::SharedString;

/// Font families by role, bundled with the application.
#[derive(Debug, Clone, PartialEq)]
pub struct ThemeFonts {
    /// Headings and the brand: Bricolage Grotesque.
    pub display: SharedString,
    /// Everything else: Instrument Sans.
    pub body: SharedString,
    /// Code, ids, paths and numbers: `JetBrains Mono`.
    pub mono: SharedString,
}

impl Default for ThemeFonts {
    fn default() -> Self {
        Self {
            display: "Bricolage Grotesque".into(),
            body: "Instrument Sans".into(),
            mono: "JetBrains Mono".into(),
        }
    }
}
