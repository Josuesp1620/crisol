use crisol_engine::dragging::DropTarget;
use crisol_engine::layout::LayoutSnapshot;
use crisol_engine::selection::ResizeHandle;
use crisol_engine::snapping::{GuideAxis, SnapGuide};
use crisol_model::geometry::Rect;
use crisol_model::identity::NodeId;
use crisol_ui::theme::ThemeColors;
use gpui::{BorderStyle, Bounds, Pixels, Window, fill, outline, point, px, quad, size};

use crate::workspace::{CanvasInteraction, DesignerWorkspace};

/// Paints what sits above the form: selection frames, resize handles, smart guides,
/// the drop marker and the dragged outline. It never draws inside the controls.
pub struct SelectionOverlay {
    pub snapshot: LayoutSnapshot,
    pub selection: Vec<NodeId>,
    pub root: NodeId,
    pub interaction: CanvasInteraction,
    pub toolbox_hover: Option<DropTarget>,
    pub colors: ThemeColors,
    pub handle_size: f32,
}

impl SelectionOverlay {
    const SELECTION_GAP: f32 = 3.;

    pub fn paint(&self, window: &mut Window) {
        for node in &self.selection {
            let Some(rect) = self.snapshot.bounds_of(*node) else {
                continue;
            };
            if *node == self.root {
                self.paint_form_handles(rect, window);
            } else {
                self.paint_selection(rect, *node == self.selection[0], window);
            }
        }
        match &self.interaction {
            CanvasInteraction::Dragging { preview, drop, .. } => {
                window.paint_quad(outline(
                    Self::bounds(preview.rect),
                    self.colors.selection,
                    BorderStyle::Dashed,
                ));
                for guide in &preview.guides {
                    self.paint_guide(guide, window);
                }
                if let Some(target) = drop {
                    self.paint_marker(target, window);
                }
            }
            CanvasInteraction::Idle | CanvasInteraction::Pressing { .. } => {}
        }
        if let Some(target) = &self.toolbox_hover {
            self.paint_marker(target, window);
        }
    }

    fn paint_selection(&self, rect: Rect, primary: bool, window: &mut Window) {
        let gap = Self::SELECTION_GAP;
        let frame = Bounds::new(
            point(px(rect.left() as f32 - gap), px(rect.top() as f32 - gap)),
            size(
                px(rect.size.width as f32 + gap * 2.),
                px(rect.size.height as f32 + gap * 2.),
            ),
        );
        window.paint_quad(quad(
            frame,
            px(2.),
            gpui::transparent_black(),
            px(1.5),
            self.colors.selection,
            BorderStyle::Solid,
        ));
        if !primary {
            return;
        }
        let half = self.handle_size / 2.;
        for handle in ResizeHandle::ALL {
            let center = handle.center_on(rect);
            let bounds = Bounds::new(
                point(px(center.x as f32 - half), px(center.y as f32 - half)),
                size(px(self.handle_size), px(self.handle_size)),
            );
            window.paint_quad(quad(
                bounds,
                px(1.),
                self.colors.surface,
                px(1.5),
                self.colors.selection,
                BorderStyle::Solid,
            ));
        }
    }

    /// The form only resizes from its right and bottom edges, as in Windows Forms.
    fn paint_form_handles(&self, rect: Rect, window: &mut Window) {
        let half = self.handle_size / 2.;
        for handle in DesignerWorkspace::FORM_HANDLES {
            let center = handle.center_on(rect);
            let bounds = Bounds::new(
                point(px(center.x as f32 - half), px(center.y as f32 - half)),
                size(px(self.handle_size), px(self.handle_size)),
            );
            window.paint_quad(quad(
                bounds,
                px(1.),
                self.colors.surface,
                px(1.5),
                self.colors.selection,
                BorderStyle::Solid,
            ));
        }
    }

    fn paint_guide(&self, guide: &SnapGuide, window: &mut Window) {
        let length = (guide.end - guide.start) as f32;
        let bounds = match guide.axis {
            GuideAxis::Vertical => Bounds::new(
                point(px(guide.position as f32), px(guide.start as f32)),
                size(px(1.), px(length)),
            ),
            GuideAxis::Horizontal => Bounds::new(
                point(px(guide.start as f32), px(guide.position as f32)),
                size(px(length), px(1.)),
            ),
        };
        window.paint_quad(fill(bounds, self.colors.accent));
    }

    fn paint_marker(&self, target: &DropTarget, window: &mut Window) {
        if let Some(marker) = target.marker {
            window.paint_quad(fill(Self::bounds(marker), self.colors.selection));
        } else if let Some(container) = self.snapshot.bounds_of(target.container) {
            window.paint_quad(outline(
                Self::bounds(container),
                self.colors.selection,
                BorderStyle::Dashed,
            ));
        }
    }

    fn bounds(rect: Rect) -> Bounds<Pixels> {
        Bounds::new(
            point(px(rect.left() as f32), px(rect.top() as f32)),
            size(px(rect.size.width as f32), px(rect.size.height as f32)),
        )
    }
}
