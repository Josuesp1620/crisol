use crisol_engine::layout::LayoutSnapshot;
use crisol_render::{DesignRenderer, LayoutRecorder};
use crisol_ui::theme::Theme;
use gpui::{
    AnyElement, Context, Div, DragMoveEvent, InteractiveElement, IntoElement, MouseButton,
    MouseDownEvent, MouseMoveEvent, ParentElement, Stateful, Styled, Window, canvas, div, px,
};

use super::dot_grid::DotGrid;
use super::form_frame::FormFrame;
use super::selection_overlay::SelectionOverlay;
use crate::designer_texts::DesignerTexts;
use crate::workspace::{DesignerWorkspace, ToolboxDrag};

/// The canvas: dotted surface, the form window and the selection adorners on top.
pub struct DesignCanvas<'workspace> {
    workspace: &'workspace DesignerWorkspace,
}

impl<'workspace> DesignCanvas<'workspace> {
    pub fn new(workspace: &'workspace DesignerWorkspace) -> Self {
        Self { workspace }
    }

    pub fn render(self, _window: &mut Window, cx: &mut Context<DesignerWorkspace>) -> AnyElement {
        let theme = Theme::of(cx).clone();
        let settings = *self.workspace.settings();
        let recorder = self.workspace.session().recorder().clone();
        recorder.begin_frame();
        let frame = self.form_frame(&recorder, cx);
        let dots = DotGrid::new(settings.canvas_dot_spacing, theme.colors.canvas_dot);
        let overlay = self.overlay(&theme);
        let surface = div()
            .id("design-canvas")
            .relative()
            .flex_1()
            .min_h_0()
            .overflow_hidden()
            .bg(theme.colors.background);
        Self::with_pointer_handlers(surface, cx)
            .child(
                canvas(
                    |_, _, _| {},
                    move |bounds, (), window, _| dots.paint(bounds, window),
                )
                .absolute()
                .size_full(),
            )
            .child(
                div()
                    .absolute()
                    .left(px(12.))
                    .top(px(9.))
                    .font_family(theme.fonts.mono.clone())
                    .text_size(px(10.5))
                    .text_color(theme.colors.muted)
                    .child(self.info()),
            )
            .child(
                div()
                    .absolute()
                    .left(settings.form_offset_x)
                    .top(settings.form_offset_y)
                    .child(frame),
            )
            .child(
                canvas(
                    |_, _, _| {},
                    move |_, (), window, _| {
                        let mut overlay = overlay;
                        overlay.snapshot = recorder.snapshot();
                        overlay.paint(window);
                    },
                )
                .absolute()
                .size_full(),
            )
            .into_any_element()
    }

    fn form_frame(
        &self,
        recorder: &LayoutRecorder,
        cx: &mut Context<DesignerWorkspace>,
    ) -> AnyElement {
        let document = self.workspace.session().document();
        let body =
            DesignRenderer::new(document, self.workspace.catalog().clone(), recorder.clone())
                .render();
        let form = document.form();
        FormFrame {
            title: form.title.clone(),
            width: px(form.size.width as f32),
            height: px(form.size.height as f32),
            title_height: self.workspace.settings().form_title_height,
        }
        .render(body, cx)
    }

    fn overlay(&self, theme: &Theme) -> SelectionOverlay {
        let session = self.workspace.session();
        SelectionOverlay {
            snapshot: LayoutSnapshot::default(),
            selection: session.selection().nodes().to_vec(),
            root: session.document().root_id(),
            interaction: self.workspace.interaction().clone(),
            toolbox_hover: self.workspace.toolbox_hover().copied(),
            colors: theme.colors,
            handle_size: f32::from(self.workspace.settings().handle_size),
        }
    }

    fn info(&self) -> String {
        let size = self.workspace.session().document().form().size;
        format!(
            "{} · {} × {} · {}",
            DesignerTexts::CANVAS_INFO,
            size.width,
            size.height,
            DesignerTexts::GRID_INFO
        )
    }

    /// Selection, moving and resizing with the mouse, and drops from the toolbox.
    fn with_pointer_handlers(
        surface: Stateful<Div>,
        cx: &mut Context<DesignerWorkspace>,
    ) -> Stateful<Div> {
        surface
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|workspace, event: &MouseDownEvent, _, cx| {
                    workspace.canvas_mouse_down(event.position, event.modifiers, cx);
                }),
            )
            .on_mouse_move(cx.listener(|workspace, event: &MouseMoveEvent, _, cx| {
                if event.pressed_button == Some(MouseButton::Left) {
                    workspace.canvas_mouse_move(event.position, cx);
                }
            }))
            .on_mouse_up(
                MouseButton::Left,
                cx.listener(|workspace, _, _, cx| workspace.canvas_mouse_up(cx)),
            )
            .on_drag_move(
                cx.listener(|workspace, event: &DragMoveEvent<ToolboxDrag>, _, cx| {
                    workspace.toolbox_drag_over(event.event.position, cx);
                }),
            )
            .on_drop(cx.listener(|workspace, drag: &ToolboxDrag, window, cx| {
                workspace.toolbox_drop(drag.kind, window.mouse_position(), cx);
            }))
    }
}
