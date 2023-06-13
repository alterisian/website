use std::collections::BTreeMap;

use relative_path::RelativePathBuf;

use super::processed_targets_count::ProcessedTargetsCount;

#[derive(Debug, thiserror::Error)]
#[error("duplicates: {0:?}")]
pub struct DuplicatesError(BTreeMap<RelativePathBuf, usize>);

impl DuplicatesError {
    pub(super) fn from_processed_targets_count(
        processed_targets_count: ProcessedTargetsCount,
    ) -> Option<Self> {
        let duplicates = processed_targets_count
            .iter()
            .filter(|(_target, &count)| count > 1)
            .map(|(target, &count)| (target.clone(), count))
            .collect::<BTreeMap<RelativePathBuf, usize>>();
        if duplicates.is_empty() {
            None
        } else {
            Some(Self(duplicates))
        }
    }
}
