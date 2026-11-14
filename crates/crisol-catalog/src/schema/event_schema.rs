/// One event a control raises, answered by a method of the user's code.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventSchema {
    pub name: &'static str,
    pub label: &'static str,
    pub description: &'static str,
}

impl EventSchema {
    pub fn new(name: &'static str, label: &'static str, description: &'static str) -> Self {
        Self {
            name,
            label,
            description,
        }
    }
}
