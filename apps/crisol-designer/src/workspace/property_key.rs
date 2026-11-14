/// One editable row of the property grid.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PropertyKey {
    Name,
    Property(String),
    Event(String),
    FormTitle,
    FormType,
    FormWidth,
    FormHeight,
}
