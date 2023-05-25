#[macro_use]
mod html;

mod components;
mod constants;
mod expected_targets;
mod file_specs;
mod fonts;
mod graphic_file_specs;
mod markdown;
mod mob;
mod pages;
mod relative_path;
mod style;
mod syn_helpers;
mod tailwind;
mod url;

use builder::OUTPUT_DIR;
use ssg_child::generate_static_site;

use anyhow::Result;
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<()> {
    let file_specs = file_specs::get().await?;

    generate_static_site(OUTPUT_DIR.clone(), file_specs)
        .map(|progress_report| {
            eprintln!("{progress_report:?}");
            progress_report
        })
        .collect::<Result<FinalReport, ()>>()
        .await
        .to_result()?;

    tailwind::execute().await?;

    Ok(())
}
