use std::time::{SystemTime, UNIX_EPOCH};

/// The tone of an output line.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputKind {
    Info,
    Success,
    Failure,
}

/// One line of the Output panel.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutputEntry {
    pub time: String,
    pub text: String,
    pub kind: OutputKind,
}

impl OutputEntry {
    const SECONDS_PER_DAY: u64 = 86_400;

    pub fn now(text: String, kind: OutputKind) -> Self {
        let seconds = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|elapsed| elapsed.as_secs())
            .unwrap_or_default()
            % Self::SECONDS_PER_DAY;
        Self {
            time: format!(
                "{:02}:{:02}:{:02}",
                seconds / 3600,
                seconds % 3600 / 60,
                seconds % 60
            ),
            text,
            kind,
        }
    }
}
