use std::collections::{BTreeMap, BTreeSet};

use relative_path::RelativePathBuf;

#[derive(Debug, Clone, thiserror::Error)]
#[error("missing targets: {0:?}")]
pub(super) struct MissingTargets(BTreeMap<RelativePathBuf, BTreeSet<RelativePathBuf>>);

impl MissingTargets {
    pub(crate) fn new(
        missing_targets: BTreeMap<RelativePathBuf, BTreeSet<RelativePathBuf>>,
    ) -> Self {
        Self(missing_targets)
    }
}
