use relative_path::RelativePathBuf;

use crate::sources::ExpectedTargets;

#[derive(Debug, Clone, getset::Getters)]
pub struct TargetSuccess {
    #[getset(get = "pub")]
    path: RelativePathBuf,
    #[getset(get = "pub")]
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
