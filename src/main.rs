use std::{
    env,
    fs::{self, File},
    path::PathBuf,
};

use anyhow::{Context, Ok};
use cucumber::{writer, World, WriterExt};
use yace::world::MyWorld;

fn create_report_file(file_path: &str) -> anyhow::Result<File> {
    let file_path = PathBuf::from(file_path);
    let reports_dir = file_path.parent().context("Unable to get parent dir")?;
    fs::create_dir_all(reports_dir)?;

    fs::File::create(&file_path).context(format!("can't create file {file_path:?}"))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let features_path = env::var("FEATURES").unwrap_or("features".to_string());
    let report_file = create_report_file("test-reports/report.xml")?;
    MyWorld::cucumber()
        .with_writer(
            writer::Basic::stdout()
                .summarized()
                .tee(writer::JUnit::for_tee(report_file, 0))
                .normalized(),
        )
        .fail_on_skipped()
        .run_and_exit(features_path)
        .await;

    Ok(())
}
