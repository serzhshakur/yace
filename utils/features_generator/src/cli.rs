use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Output directory for features files
    #[arg(short, long)]
    pub output_dir: PathBuf,
    /// Number of features to generate
    #[arg(long, value_parser = clap::value_parser!(u16).range(5..5000))]
    pub features_count: u16,
    /// Max delay in seconds
    #[arg(long, default_value_t = 10, value_parser = clap::value_parser!(u16).range(1..))]
    pub max_delay: u16,
    /// Max number of scenarios in a feature
    #[arg(long, short = 's', default_value_t = 10, value_parser = clap::value_parser!(u16).range(2..))]
    pub max_scenarios_in_file: u16,
}
