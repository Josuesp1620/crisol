use std::ops::Range;

/// The kind of a highlighted piece of Rust.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodeToken {
    Keyword,
    String,
    Comment,
    Number,
    Type,
}

impl CodeToken {
    /// A highlighted range of one line.
    pub fn span(self, range: Range<usize>) -> (Range<usize>, CodeToken) {
        (range, self)
    }
}
