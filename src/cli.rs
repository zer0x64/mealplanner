use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
pub struct Cli {
    #[arg(default_value = "./recipes.ron")]
    pub file: PathBuf,
}
