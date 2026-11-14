use super::{OutputEntry, OutputKind};

/// What the designer did, newest last, as shown in the Output panel.
#[derive(Debug, Default)]
pub struct OutputLog {
    entries: Vec<OutputEntry>,
}

impl OutputLog {
    const CAPACITY: usize = 200;

    pub fn info(&mut self, text: impl Into<String>) {
        self.push(text.into(), OutputKind::Info);
    }

    pub fn success(&mut self, text: impl Into<String>) {
        self.push(text.into(), OutputKind::Success);
    }

    pub fn failure(&mut self, text: impl Into<String>) {
        self.push(text.into(), OutputKind::Failure);
    }

    pub fn entries(&self) -> &[OutputEntry] {
        &self.entries
    }

    fn push(&mut self, text: String, kind: OutputKind) {
        if self.entries.len() == Self::CAPACITY {
            self.entries.remove(0);
        }
        self.entries.push(OutputEntry::now(text, kind));
    }
}
