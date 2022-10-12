use std::{
    fs::{self, File},
    path::PathBuf,
};

use anyhow::{Context, Ok};
use cucumber::{given, then, when, writer, World, WriterExt as _};

#[derive(Debug, World, Default)]
#[world(init = Self::new)]
pub struct MyWorld {
    count: i32,
}

impl MyWorld {
    fn new() -> Self {
        Self { count: 0 }
    }
}

#[given(expr = "I start with {int} items")]
async fn init(world: &mut MyWorld, count: i32) {
    world.count = count;
}

#[when(expr = "I add {int} items")]
async fn add_count(world: &mut MyWorld, count: i32) {
    world.count += count;
}

#[then(expr = "I have {int} items")]
async fn check_result(world: &mut MyWorld, count: i32) {
    assert_eq!(world.count, count);
}

fn create_report_file(file_path: &str) -> anyhow::Result<File> {
    let file_path = PathBuf::from(file_path);
    let reports_dir = file_path.parent().context("Unable to get parent dir")?;
    fs::create_dir_all(&reports_dir).context(format!("can't create dir {reports_dir:?}"))?;

    fs::File::create(&file_path).context(format!("can't create file {file_path:?}"))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let report_file = create_report_file("test-reports/report.xml")?;
    MyWorld::cucumber()
        .with_writer(
            writer::Basic::stdout()
                .summarized()
                .tee(writer::JUnit::for_tee(report_file, 0))
                .normalized(),
        )
        .run_and_exit("tests/features")
        .await;

    Ok(())
}
