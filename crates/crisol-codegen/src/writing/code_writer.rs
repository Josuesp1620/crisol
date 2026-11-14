/// Builds source text line by line with consistent indentation.
pub struct CodeWriter {
    text: String,
    level: usize,
    indent_width: usize,
}

impl CodeWriter {
    pub fn new(indent_width: usize) -> Self {
        Self {
            text: String::new(),
            level: 0,
            indent_width,
        }
    }

    pub fn line(&mut self, content: &str) {
        if !content.is_empty() {
            self.text
                .push_str(&" ".repeat(self.level * self.indent_width));
            self.text.push_str(content);
        }
        self.text.push('\n');
    }

    pub fn blank(&mut self) {
        self.text.push('\n');
    }

    /// Writes `opening`, indents what `body` writes, then writes `closing`.
    pub fn block(&mut self, opening: &str, closing: &str, body: impl FnOnce(&mut CodeWriter)) {
        self.line(opening);
        self.level += 1;
        body(self);
        self.level -= 1;
        self.line(closing);
    }

    pub fn indent(&mut self) {
        self.level += 1;
    }

    pub fn dedent(&mut self) {
        self.level = self.level.saturating_sub(1);
    }

    pub fn finish(self) -> String {
        self.text
    }
}
