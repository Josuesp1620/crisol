use crisol_model::identity::NodeId;

/// One connected event: which control raises it and which method answers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventBinding {
    pub node: NodeId,
    pub node_name: String,
    pub kind: String,
    pub event: String,
    pub event_label: String,
    pub handler: String,
}
