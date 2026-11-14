use gpui::{AnyElement, App, IntoElement, ParentElement, RenderOnce, Styled, Window, div, px};

use super::Length;
use super::grid_slot::GridSlot;

/// Places children in rows and columns. The first column can have a fixed width, which
/// keeps a column of labels aligned; the others share the remaining space.
#[derive(IntoElement)]
pub struct Grid {
    columns: u32,
    first_column_width: f32,
    row_gap: f32,
    column_gap: f32,
    padding: f32,
    slots: Vec<GridSlot>,
}

impl Grid {
    pub fn new(columns: u32) -> Self {
        Self {
            columns: columns.max(1),
            first_column_width: 0.,
            row_gap: 0.,
            column_gap: 0.,
            padding: 0.,
            slots: Vec::new(),
        }
    }

    pub fn first_column_width(mut self, pixels: f32) -> Self {
        self.first_column_width = pixels;
        self
    }

    pub fn row_gap(mut self, pixels: f32) -> Self {
        self.row_gap = pixels;
        self
    }

    /// Space between columns.
    pub fn gap(mut self, pixels: f32) -> Self {
        self.column_gap = pixels;
        self
    }

    pub fn padding(mut self, pixels: f32) -> Self {
        self.padding = pixels;
        self
    }

    pub fn cell(
        mut self,
        row: u32,
        column: u32,
        column_span: u32,
        width: Length,
        element: impl IntoElement,
    ) -> Self {
        self.slots.push(GridSlot {
            row,
            column,
            column_span: column_span.max(1),
            width,
            element: element.into_any_element(),
        });
        self
    }

    fn row_count(&self) -> u32 {
        let mut count = 0;
        for slot in &self.slots {
            count = count.max(slot.row + 1);
        }
        count
    }

    fn column_box(&self, column: u32, span: u32) -> gpui::Div {
        let mut cell = div().flex().items_center().min_w_0();
        if column == 0 && self.first_column_width > 0. && span == 1 {
            cell = cell.w(px(self.first_column_width)).flex_none();
        } else {
            cell = cell.flex_1();
            #[allow(
                clippy::cast_precision_loss,
                reason = "a column span is a small number"
            )]
            let grow = span as f32;
            cell.style().flex_grow = Some(grow);
        }
        cell
    }
}

impl RenderOnce for Grid {
    fn render(mut self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let rows = self.row_count();
        let mut cells: Vec<Option<GridSlot>> = Vec::new();
        for _ in 0..(rows * self.columns) {
            cells.push(None);
        }
        for slot in std::mem::take(&mut self.slots) {
            let position = (slot.row * self.columns + slot.column.min(self.columns - 1)) as usize;
            cells[position] = Some(slot);
        }
        let mut grid = div()
            .flex()
            .flex_col()
            .w_full()
            .gap(px(self.row_gap))
            .p(px(self.padding));
        for row in 0..rows {
            let mut line = div()
                .flex()
                .items_center()
                .w_full()
                .gap(px(self.column_gap));
            let mut column = 0;
            while column < self.columns {
                let position = (row * self.columns + column) as usize;
                let (span, content): (u32, Option<AnyElement>) = match cells[position].take() {
                    Some(slot) => {
                        let span = slot.column_span.min(self.columns - column);
                        let holder = slot.width.apply_width(div().flex(), false);
                        (span, Some(holder.child(slot.element).into_any_element()))
                    }
                    None => (1, None),
                };
                let mut cell = self.column_box(column, span);
                if let Some(element) = content {
                    cell = cell.child(element);
                }
                line = line.child(cell);
                column += span;
            }
            grid = grid.child(line);
        }
        grid
    }
}
