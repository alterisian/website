mod duplicates_report;
mod failed_targets;
mod missing_targets;
mod processed_targets_count;

use std::fmt::Display;

use relative_path::RelativePathBuf;

use crate::{sources::ExpectedTargets, target_success::TargetSuccess, TargetError};

use self::{
    duplicates_report::DuplicatesError, failed_targets::FailedTargets,
    missing_targets::MissingTargets, processed_targets_count::ProcessedTargetsCount,
};

impl FinalError {
    pub fn duplicates(&self) -> Option<DuplicatesError> {
        DuplicatesError::from_processed_targets_count(&self.processed_targets_count)
    }
}

#[derive(Debug, Clone, Default, thiserror::Error)]
pub struct FinalError {
    // Keys are the paths of processed targets.
    // Values are the number of times a target has been processed.
    processed_targets_count: ProcessedTargetsCount,

    // A report on missing expected targets.
    //
    // Keys are missing expected targets.
    // Values are a set of targets that expect that key target.
    missing_targets: MissingTargets,

    // List of targets with failures.
    failed_targets: FailedTargets,
}

impl Display for FinalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

#[derive(Debug, Clone, Default)]
pub struct FinalErrorBuilder {
    // Keys are the paths of processed targets.
    // Values are the number of times a target has been processed.
    processed_targets_count: ProcessedTargetsCount,

    // A report on missing expected targets.
    //
    // Keys are missing expected targets.
    // Values are a set of targets that expect that key target.
    missing_targets: MissingTargets,

    // List of targets with failures.
    failed_targets: FailedTargets,
}
impl FinalErrorBuilder {
    pub(crate) fn add(self, processing_result: Result<TargetSuccess, TargetError>) -> Self {
        let (target, expected_targets) = match &processing_result {
            Ok(success) => success.clone(),
            Err(target_error) => {
                self.failed_targets
                    .insert(target_error.spec_target_path().clone());
                (
                    target_error.spec_target_path().clone(),
                    ExpectedTargets::default(),
                )
            }
        };

        expected_targets.into_iter().for_each(|expected_target| {
            self.missing_targets
                .entry(expected_target)
                .or_default()
                .insert(target.clone());
        });

        self.missing_targets.remove(&target);

        *self.processed_targets_count.entry(target).or_default() += 1;

        processing_result
    }
}
