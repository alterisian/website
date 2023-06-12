mod disk_caching_http_client;
mod file_spec;
pub mod final_error;
pub mod sources;
pub mod target_error;
pub mod target_success;

use camino::Utf8PathBuf;
pub use file_spec::FileSpec;
use futures::{stream, Stream, StreamExt};
use relative_path::RelativePathBuf;

use sources::FileSource;
use target_error::{TargetError, TargetErrorCause};
use target_success::TargetSuccess;

/// Panics on duplicate `FileSpec` targets
pub fn generate_static_site(
    output_dir: Utf8PathBuf,
    file_specs: impl IntoIterator<Item = FileSpec>,
) -> impl Stream<Item = Result<TargetSuccess, TargetError>> {
    stream::iter(file_specs)
        .map(move |file_spec| file_spec.generate(output_dir.clone()))
        .buffer_unordered(usize::MAX)
}
