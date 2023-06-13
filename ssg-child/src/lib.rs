mod disk_caching_http_client;
mod file_spec;
pub mod final_error;
pub mod generation_task;
pub mod sources;
pub mod target_error;
pub mod target_success;

use camino::Utf8PathBuf;
pub use file_spec::FileSpec;
use futures::{stream::{self, BufferUnordered}, StreamExt};
use generation_task::GenerationTask;

use target_error::TargetError;

type Stream = ();

/// Panics on duplicate `FileSpec` targets
pub fn generate_static_site(
    output_dir: Utf8PathBuf,
    file_specs: impl IntoIterator<Item = FileSpec>,
) -> GenerationTask {
    let stream: Stream = stream::iter(file_specs)
        .map(move |file_spec| file_spec.generate(output_dir.clone()))
        .buffer_unordered(usize::MAX);

    GenerationTask::new(stream)
}
