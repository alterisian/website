use std::collections::BTreeSet;

use relative_path::RelativePathBuf;

#[derive(Debug, Clone, Default)]
pub(super) struct FailedTargets(BTreeSet<RelativePathBuf>);

impl FailedTargets {
    pub(crate) fn new(failed_targets: BTreeSet<RelativePathBuf>) -> Self {
        Self(failed_targets)
    }
}
