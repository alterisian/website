use relative_path::RelativePathBuf;

use crate::sources::ExpectedTargets;

#[derive(Debug, Clone)]
pub struct TargetSuccess {
    path: RelativePathBuf,
    expected_targets: ExpectedTargets,
}

impl TargetSuccess {
    pub(super) fn new(path: RelativePathBuf, expected_targets: Option<ExpectedTargets>) -> Self {
        Self {
            path,
            expected_targets: expected_targets.unwrap_or_default(),
        }
    }
}
