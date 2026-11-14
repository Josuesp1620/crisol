use std::ops::Range;

use unicode_segmentation::UnicodeSegmentation;

use super::InputKind;

/// The text of a text box and its selection, with every editing rule and no drawing.
/// Offsets are UTF-8 byte offsets that always fall on character boundaries.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TextEditState {
    content: String,
    selection: Range<usize>,
    reversed: bool,
    marked: Option<Range<usize>>,
    max_length: usize,
    input_kind: InputKind,
}

impl TextEditState {
    pub fn new(input_kind: InputKind, max_length: usize) -> Self {
        Self {
            max_length,
            input_kind,
            ..Self::default()
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn selection(&self) -> Range<usize> {
        self.selection.clone()
    }

    pub fn marked(&self) -> Option<Range<usize>> {
        self.marked.clone()
    }

    pub fn input_kind(&self) -> InputKind {
        self.input_kind
    }

    pub fn set_input_kind(&mut self, input_kind: InputKind) {
        self.input_kind = input_kind;
    }

    pub fn set_max_length(&mut self, max_length: usize) {
        self.max_length = max_length;
    }

    /// Where the caret is: the moving end of the selection.
    pub fn cursor(&self) -> usize {
        if self.reversed {
            self.selection.start
        } else {
            self.selection.end
        }
    }

    pub fn set_content(&mut self, text: &str) {
        self.content = self.filtered(text, usize::MAX);
        self.move_to(self.content.len());
        self.marked = None;
    }

    pub fn move_to(&mut self, offset: usize) {
        let offset = self.clamped(offset);
        self.selection = offset..offset;
        self.reversed = false;
    }

    pub fn select_to(&mut self, offset: usize) {
        let offset = self.clamped(offset);
        if self.reversed {
            self.selection.start = offset;
        } else {
            self.selection.end = offset;
        }
        if self.selection.end < self.selection.start {
            self.reversed = !self.reversed;
            self.selection = self.selection.end..self.selection.start;
        }
    }

    pub fn set_selection(&mut self, range: Range<usize>) {
        self.selection = self.clamped(range.start)..self.clamped(range.end);
        self.reversed = false;
    }

    pub fn select_all(&mut self) {
        self.move_to(0);
        self.select_to(self.content.len());
    }

    pub fn selected_text(&self) -> &str {
        &self.content[self.selection.clone()]
    }

    pub fn previous_boundary(&self, offset: usize) -> usize {
        let mut boundary = 0;
        for (index, _) in self.content.grapheme_indices(true) {
            if index >= offset {
                break;
            }
            boundary = index;
        }
        boundary
    }

    pub fn next_boundary(&self, offset: usize) -> usize {
        for (index, _) in self.content.grapheme_indices(true) {
            if index > offset {
                return index;
            }
        }
        self.content.len()
    }

    /// Replaces `range` (or the marked text, or the selection) with `text`, keeping only
    /// what the input kind accepts and the maximum length allows. Returns whether the
    /// content changed.
    pub fn replace(&mut self, range: Option<Range<usize>>, text: &str) -> bool {
        let range = self.target_range(range);
        let room = self.room_for(&range);
        let accepted = self.filtered(text, room);
        let before = self.content.clone();
        self.content.replace_range(range.clone(), &accepted);
        let caret = range.start + accepted.len();
        self.selection = caret..caret;
        self.reversed = false;
        self.marked = None;
        before != self.content
    }

    /// Replaces like `replace` but keeps the new text marked, as IME composition does.
    pub fn replace_and_mark(
        &mut self,
        range: Option<Range<usize>>,
        text: &str,
        selected: Option<Range<usize>>,
    ) -> bool {
        let start = self.target_range(range.clone()).start;
        let changed = self.replace(range, text);
        let inserted = self.content.len().min(start + text.len()) - start;
        self.marked = if inserted > 0 {
            Some(start..start + inserted)
        } else {
            None
        };
        if let Some(selected) = selected {
            self.selection =
                start + selected.start.min(inserted)..start + selected.end.min(inserted);
        }
        changed
    }

    pub fn unmark(&mut self) {
        self.marked = None;
    }

    pub fn utf16_length(&self) -> usize {
        self.content.encode_utf16().count()
    }

    pub fn offset_to_utf16(&self, offset: usize) -> usize {
        self.content[..self.clamped(offset)].encode_utf16().count()
    }

    pub fn offset_from_utf16(&self, utf16_offset: usize) -> usize {
        let mut counted = 0;
        for (index, character) in self.content.char_indices() {
            if counted >= utf16_offset {
                return index;
            }
            counted += character.len_utf16();
        }
        self.content.len()
    }

    pub fn range_to_utf16(&self, range: &Range<usize>) -> Range<usize> {
        self.offset_to_utf16(range.start)..self.offset_to_utf16(range.end)
    }

    pub fn range_from_utf16(&self, range: &Range<usize>) -> Range<usize> {
        self.offset_from_utf16(range.start)..self.offset_from_utf16(range.end)
    }

    fn target_range(&self, range: Option<Range<usize>>) -> Range<usize> {
        let chosen = match (range, &self.marked) {
            (Some(range), _) => range,
            (None, Some(marked)) => marked.clone(),
            (None, None) => self.selection.clone(),
        };
        self.clamped(chosen.start)..self.clamped(chosen.end)
    }

    /// How many characters can still be added once `range` is replaced.
    fn room_for(&self, range: &Range<usize>) -> usize {
        if self.max_length == 0 {
            return usize::MAX;
        }
        let kept = self.content.chars().count() - self.content[range.clone()].chars().count();
        self.max_length.saturating_sub(kept)
    }

    fn filtered(&self, text: &str, room: usize) -> String {
        let mut accepted = String::with_capacity(text.len());
        let mut count = 0;
        for character in text.chars() {
            if count == room {
                break;
            }
            if self.input_kind.accepts(character) {
                accepted.push(character);
                count += 1;
            }
        }
        accepted
    }

    fn clamped(&self, offset: usize) -> usize {
        let mut offset = offset.min(self.content.len());
        while !self.content.is_char_boundary(offset) {
            offset -= 1;
        }
        offset
    }
}
