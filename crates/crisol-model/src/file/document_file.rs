use serde::{Deserialize, Serialize};

use crate::document::FormInfo;

use super::node_record::NodeRecord;

/// The top level of a `.crisol` file. Field order here is the order written to disk.
#[derive(Debug, Serialize, Deserialize)]
pub(super) struct DocumentFile {
    pub format: String,
    pub form: FormInfo,
    pub root: NodeRecord,
}
