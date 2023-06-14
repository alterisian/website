use getset::Getters;
use relative_path::RelativePathBuf;

#[derive(Debug, thiserror::Error, Getters)]
#[error("Failed to generate {spec_target_path}: {source}")]
pub(crate) struct TargetError {
    #[getset(get = "pub")]
    spec_target_path: RelativePathBuf,
    #[getset(get = "pub")]
    source: TargetErrorCause,
}

impl TargetError {
    pub(crate) fn new(spec_target_path: RelativePathBuf, source: TargetErrorCause) -> Self {
        Self {
            spec_target_path,
            source,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum TargetErrorCause {
    #[error(transparent)]
    Source(Box<dyn std::error::Error + Send>),
    #[error(transparent)]
    TargetIo(#[from] std::io::Error),
}
