mod disk_caching_http_client;
mod file_spec;
pub mod final_error;
pub mod generation_task;
pub mod sources;
pub mod target_error;
pub mod target_success;

use camino::Utf8PathBuf;
pub use file_spec::FileSpec;
use final_error::FinalError;
use futures::{stream, Stream, StreamExt};
use generation_task::GenerationTask;
use relative_path::RelativePathBuf;

use sources::FileSource;
use target_error::{TargetError, TargetErrorCause};
use target_success::TargetSuccess;

/// Panics on duplicate `FileSpec` targets
pub fn generate_static_site(
    output_dir: Utf8PathBuf,
    file_specs: impl IntoIterator<Item = FileSpec>,
) -> GenerationTask<impl Stream<Item = Result<TargetSuccess, TargetError>>> {
    GenerationTask::new(
        stream::iter(file_specs)
            .map(move |file_spec| file_spec.generate(output_dir.clone()))
            .buffer_unordered(usize::MAX),
    )
}
