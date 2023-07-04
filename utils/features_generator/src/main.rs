use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::Context;
use clap::Parser;
use cli::Cli;
use rand::Rng;
mod cli;

fn create_features_dir(path: &PathBuf) -> anyhow::Result<()> {
    fs::remove_dir_all(path).unwrap_or_default();
    fs::create_dir_all(path)?;
    Ok(())
}

fn create_feature_file(features_dir: &Path, file_name: &str) -> anyhow::Result<PathBuf> {
    let feature_file = features_dir.join(file_name);
    fs::File::create(&feature_file).context("can't create file".to_string())?;
    Ok(feature_file)
}

fn feature(f_idx: u16) -> String {
    format!(
        "Feature: My test {f_idx}

  Background:
    Given I start with {f_idx} items
"
    )
}

fn scenario(f_idx: u16, sc_idx: u16, delay: u16) -> String {
    let result = f_idx + sc_idx;
    format!(
        "
  Scenario: My scenario {sc_idx} ({f_idx}) delayed for {delay}s
    When I pause for {delay} second
    And I add {sc_idx} items
    Then I have {result} items
"
    )
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    create_features_dir(&cli.output_dir)?;

    for f_idx in 1..=cli.features_count {
        let mut rng = rand::thread_rng();
        let mut feature = feature(f_idx);
        let sc_count = rng.gen_range(2..=cli.max_scenarios_in_file);
        for sc_idx in 1..sc_count {
            let rnd_delay = rng.gen_range(0..=cli.max_delay);
            let scenario = scenario(f_idx, sc_idx, rnd_delay);
            feature += &scenario;
        }
        let feature_file = create_feature_file(&cli.output_dir, &format!("feat_{f_idx}.feature"))?;
        fs::write(feature_file, feature)?;
    }

    Ok(())
}
