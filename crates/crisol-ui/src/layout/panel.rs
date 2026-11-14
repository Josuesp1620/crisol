use gpui::{App, Div, IntoElement, ParentElement, RenderOnce, Styled, Window, div, px};

use super::Anchors;
use super::panel_slot::PanelSlot;

/// Places children at fixed positions. When the panel grows past its designed size,
/// children keep their distance to the edges they are anchored to. Desktop only.
#[derive(IntoElement)]
pub struct Panel {
    width: f32,
    height: f32,
    slots: Vec<PanelSlot>,
}

impl Panel {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            width,
            height,
            slots: Vec::new(),
        }
    }

    pub fn place(
        mut self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        anchors: Anchors,
        element: impl IntoElement,
    ) -> Self {
        self.slots.push(PanelSlot {
            x,
            y,
            width,
            height,
            anchors,
            element: element.into_any_element(),
        });
        self
    }

    fn horizontal(&self, slot: &PanelSlot, holder: Div) -> Div {
        let right_gap = px(self.width - slot.x - slot.width);
        match (slot.anchors.left, slot.anchors.right) {
            (true, true) => holder.left(px(slot.x)).right(right_gap),
            (false, true) => holder.right(right_gap).w(px(slot.width)),
            _ => holder.left(px(slot.x)).w(px(slot.width)),
        }
    }

    fn vertical(&self, slot: &PanelSlot, holder: Div) -> Div {
        let bottom_gap = px(self.height - slot.y - slot.height);
        match (slot.anchors.top, slot.anchors.bottom) {
            (true, true) => holder.top(px(slot.y)).bottom(bottom_gap),
            (false, true) => holder.bottom(bottom_gap).h(px(slot.height)),
            _ => holder.top(px(slot.y)).h(px(slot.height)),
        }
    }
}

impl RenderOnce for Panel {
    fn render(mut self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut panel = div().relative().min_w(px(self.width)).h(px(self.height));
        for slot in std::mem::take(&mut self.slots) {
            let holder = self.vertical(&slot, self.horizontal(&slot, div().absolute().flex()));
            panel = panel.child(holder.child(slot.element));
        }
        panel
    }
}
