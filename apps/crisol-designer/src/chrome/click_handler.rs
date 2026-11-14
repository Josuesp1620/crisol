use gpui::{App, ClickEvent, Window};

/// A boxed click listener, as panels pass them to the pieces they draw.
pub type ClickHandler = Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>;
