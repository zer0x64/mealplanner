use std::{fs::File, io::BufReader, path::PathBuf};

use clap::Parser;
use rand::seq::SliceRandom;
use serde::Deserialize;

#[derive(Debug, Parser)]
struct Cli {
    #[arg(default_value = "./recipes.ron")]
    file: PathBuf,

    #[arg(short, long, default_value_t = 3)]
    count: usize,
}

#[derive(Debug, Deserialize)]
struct Recipe {
    name: String,
    ingredients: Vec<String>,
}

fn main() {
    let cli = Cli::parse();

    let file = File::open(cli.file).expect("File does not exist");
    let recipes: Vec<Recipe> =
        ron::de::from_reader(BufReader::new(file)).expect("File format is invalid");

    let chosen_recipe = recipes.choose_multiple(&mut rand::thread_rng(), cli.count);

    for (i, r) in chosen_recipe.enumerate() {
        println!("Recipe {i}: {}", r.name);

        println!("Ingredients:\n");

        for ingredient in &r.ingredients {
            println!("{}", ingredient)
        }
    }
}
