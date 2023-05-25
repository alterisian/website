use std::collections::BTreeSet;

use relative_path::RelativePathBuf;

#[derive(Debug, Clone, Default)]
pub(super) struct FailedTargets(BTreeSet<RelativePathBuf>);
impl FailedTargets {
    pub(crate) fn insert(&mut self, clone: RelativePathBuf) {
        self.0.insert(clone);
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
