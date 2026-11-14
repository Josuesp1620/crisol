use gpui::{
    App, Bounds, Element, ElementId, ElementInputHandler, Entity, GlobalElementId, LayoutId,
    PaintQuad, Pixels, ShapedLine, Style, TextAlign, TextRun, UnderlineStyle, Window, fill, point,
    px, relative, size,
};

use super::TextBox;
use crate::theme::Theme;

/// Draws the text, selection and caret of a text box and connects it to the platform's
/// text input, so IME composition works.
pub struct TextBoxElement {
    text_box: Entity<TextBox>,
}

pub struct TextBoxPaintState {
    line: Option<ShapedLine>,
    caret: Option<PaintQuad>,
    selection: Option<PaintQuad>,
}

impl TextBoxElement {
    pub fn new(text_box: Entity<TextBox>) -> Self {
        Self { text_box }
    }

    fn runs(&self, length: usize, color: gpui::Hsla, window: &Window, cx: &App) -> Vec<TextRun> {
        let text_box = self.text_box.read(cx);
        let font = window.text_style().font();
        let base = TextRun {
            len: length,
            font,
            color,
            background_color: None,
            underline: None,
            strikethrough: None,
        };
        let Some(marked) = text_box.edit_state().marked() else {
            return vec![base];
        };
        let start = text_box.display_offset(marked.start);
        let end = text_box.display_offset(marked.end);
        let mut runs = Vec::new();
        for (run_length, underlined) in [(start, false), (end - start, true), (length - end, false)]
        {
            if run_length == 0 {
                continue;
            }
            let mut run = base.clone();
            run.len = run_length;
            if underlined {
                run.underline = Some(UnderlineStyle {
                    color: Some(color),
                    thickness: px(1.),
                    wavy: false,
                });
            }
            runs.push(run);
        }
        runs
    }
}

impl Element for TextBoxElement {
    type RequestLayoutState = ();
    type PrepaintState = TextBoxPaintState;

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static core::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let mut style = Style::default();
        style.size.width = relative(1.).into();
        style.size.height = window.line_height().into();
        (window.request_layout(style, [], cx), ())
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        let theme = Theme::of(cx).clone();
        let text_box = self.text_box.read(cx);
        let state = text_box.edit_state().clone();
        let (text, color) = if state.content().is_empty() {
            (text_box.placeholder_text(), theme.colors.muted)
        } else {
            (text_box.display_text(), window.text_style().color)
        };
        let caret_offset = text_box.display_offset(state.cursor());
        let selection_start = text_box.display_offset(state.selection().start);
        let selection_end = text_box.display_offset(state.selection().end);
        let runs = if state.content().is_empty() {
            vec![TextRun {
                len: text.len(),
                font: window.text_style().font(),
                color,
                background_color: None,
                underline: None,
                strikethrough: None,
            }]
        } else {
            self.runs(text.len(), color, window, cx)
        };
        let font_size = window.text_style().font_size.to_pixels(window.rem_size());
        let line = window
            .text_system()
            .shape_line(text, font_size, &runs, None);
        let (selection, caret) = if state.selection().is_empty() {
            let x = if state.content().is_empty() {
                px(0.)
            } else {
                line.x_for_index(caret_offset)
            };
            let caret = fill(
                Bounds::new(
                    point(bounds.left() + x, bounds.top()),
                    size(px(1.5), bounds.size.height),
                ),
                theme.colors.ink,
            );
            (None, Some(caret))
        } else {
            let selection = fill(
                Bounds::from_corners(
                    point(
                        bounds.left() + line.x_for_index(selection_start),
                        bounds.top(),
                    ),
                    point(
                        bounds.left() + line.x_for_index(selection_end),
                        bounds.bottom(),
                    ),
                ),
                theme.colors.selection_soft,
            );
            (Some(selection), None)
        };
        TextBoxPaintState {
            line: Some(line),
            caret,
            selection,
        }
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        let focus = self.text_box.read(cx).focus().clone();
        window.handle_input(
            &focus,
            ElementInputHandler::new(bounds, self.text_box.clone()),
            cx,
        );
        if let Some(selection) = prepaint.selection.take() {
            window.paint_quad(selection);
        }
        let Some(line) = prepaint.line.take() else {
            return;
        };
        if line
            .paint(
                bounds.origin,
                window.line_height(),
                TextAlign::Left,
                None,
                window,
                cx,
            )
            .is_err()
        {
            return;
        }
        if self.text_box.read(cx).is_focused(window)
            && let Some(caret) = prepaint.caret.take()
        {
            window.paint_quad(caret);
        }
        self.text_box.update(cx, |text_box, _cx| {
            text_box.last_line = Some(line);
            text_box.last_bounds = Some(bounds);
        });
    }
}
