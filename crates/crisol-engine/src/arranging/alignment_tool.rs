use crisol_commands::Command;
use crisol_commands::commands::{SetPlacementCommand, Transaction};
use crisol_model::document::FormDocument;
use crisol_model::geometry::{Point, Rect};
use crisol_model::identity::NodeId;
use crisol_model::layout::{AnchorSet, Placement};

use super::Alignment;

/// Lines up controls placed at absolute positions in the same panel. Controls in stacks and
/// grids are placed by their container, so they cannot be aligned by hand.
pub struct AlignmentTool<'document> {
    document: &'document FormDocument,
}

impl<'document> AlignmentTool<'document> {
    pub fn new(document: &'document FormDocument) -> Self {
        Self { document }
    }

    /// Whether `selection` holds at least two absolute controls sharing one panel.
    pub fn can_align(&self, selection: &[NodeId]) -> bool {
        self.absolute_rects(selection).len() >= 2
    }

    /// One transaction moving or resizing every selected control, aligned to the first one.
    pub fn arrange(&self, selection: &[NodeId], alignment: Alignment) -> Option<Transaction> {
        let rects = self.absolute_rects(selection);
        if rects.len() < 2 {
            return None;
        }
        let targets = match alignment {
            Alignment::SpaceHorizontally => Self::spaced(&rects),
            _ => Self::aligned(&rects, alignment),
        };
        let mut commands: Vec<Box<dyn Command>> = Vec::new();
        for (node, rect, anchor) in targets {
            commands.push(Box::new(SetPlacementCommand {
                node,
                placement: Placement::Absolute { rect, anchor },
            }));
        }
        Some(Transaction::new(alignment.label(), commands))
    }

    fn aligned(
        rects: &[(NodeId, Rect, AnchorSet)],
        alignment: Alignment,
    ) -> Vec<(NodeId, Rect, AnchorSet)> {
        let primary = rects[0].1;
        let mut result = Vec::with_capacity(rects.len());
        for (node, rect, anchor) in rects {
            let moved = match alignment {
                Alignment::Left => rect.moved_to(Point::new(primary.left(), rect.top())),
                Alignment::Center => rect.moved_to(Point::new(
                    primary.center_x() - rect.size.width / 2,
                    rect.top(),
                )),
                Alignment::Right => {
                    rect.moved_to(Point::new(primary.right() - rect.size.width, rect.top()))
                }
                Alignment::Top => rect.moved_to(Point::new(rect.left(), primary.top())),
                Alignment::Bottom => {
                    rect.moved_to(Point::new(rect.left(), primary.bottom() - rect.size.height))
                }
                Alignment::SameSize => rect.resized_to(primary.size),
                Alignment::SpaceHorizontally => *rect,
            };
            result.push((*node, moved, anchor.clone()));
        }
        result
    }

    fn spaced(rects: &[(NodeId, Rect, AnchorSet)]) -> Vec<(NodeId, Rect, AnchorSet)> {
        let mut ordered = rects.to_vec();
        ordered.sort_by_key(|(_, rect, _)| rect.left());
        let first = ordered[0].1;
        let last = ordered[ordered.len() - 1].1;
        let mut widths = 0;
        for (_, rect, _) in &ordered {
            widths += rect.size.width;
        }
        let gaps = i32::try_from(ordered.len() - 1).unwrap_or(1);
        let gap = (last.right() - first.left() - widths) / gaps;
        let mut x = first.left();
        let mut result = Vec::with_capacity(ordered.len());
        for (node, rect, anchor) in ordered {
            result.push((node, rect.moved_to(Point::new(x, rect.top())), anchor));
            x += rect.size.width + gap;
        }
        result
    }

    fn absolute_rects(&self, selection: &[NodeId]) -> Vec<(NodeId, Rect, AnchorSet)> {
        let mut rects = Vec::new();
        let mut parent = None;
        for node in selection {
            let Ok(found) = self.document.node(*node) else {
                continue;
            };
            let Placement::Absolute { rect, anchor } = &found.placement else {
                continue;
            };
            if parent.is_some() && parent != found.parent() {
                continue;
            }
            parent = found.parent();
            rects.push((*node, *rect, anchor.clone()));
        }
        rects
    }
}
