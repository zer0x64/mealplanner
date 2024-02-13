use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
pub struct Cli {
    #[arg(default_value = "./mealplanner.ron")]
    pub file: PathBuf,
}
