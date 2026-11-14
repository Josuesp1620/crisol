use gpui::{Bounds, Hsla, Pixels, Window, fill, point, px, size};

/// Paints the dotted background of the canvas.
pub struct DotGrid {
    spacing: Pixels,
    color: Hsla,
}

impl DotGrid {
    pub fn new(spacing: Pixels, color: Hsla) -> Self {
        Self { spacing, color }
    }

    pub fn paint(&self, bounds: Bounds<Pixels>, window: &mut Window) {
        let step = f32::from(self.spacing);
        let mut y = f32::from(bounds.top()) + step / 2.;
        while y < f32::from(bounds.bottom()) {
            let mut x = f32::from(bounds.left()) + step / 2.;
            while x < f32::from(bounds.right()) {
                window.paint_quad(fill(
                    Bounds::new(point(px(x), px(y)), size(px(1.), px(1.))),
                    self.color,
                ));
                x += step;
            }
            y += step;
        }
    }
}
