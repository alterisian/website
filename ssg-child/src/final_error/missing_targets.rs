use std::collections::{btree_map::Entry, BTreeMap, BTreeSet};

use relative_path::{RelativePath, RelativePathBuf};

#[derive(Debug, Clone, Default)]
pub(super) struct MissingTargets(BTreeMap<RelativePathBuf, BTreeSet<RelativePathBuf>>);
impl MissingTargets {
    pub(crate) fn entry(
        &mut self,
        path: RelativePathBuf,
    ) -> Entry<RelativePathBuf, BTreeSet<RelativePathBuf>> {
        self.0.entry(path)
    }

    pub(crate) fn remove<P: AsRef<RelativePath>>(&mut self, path: P) {
        self.0.remove(path.as_ref());
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
