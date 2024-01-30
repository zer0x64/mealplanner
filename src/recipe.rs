use std::{collections::HashMap, fs::OpenOptions, io::BufWriter, path::PathBuf};

use ron::ser::PrettyConfig;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Clone)]
pub struct Recipe {
    pub name: String,
    pub ingredients: Vec<String>,
}

#[derive(Clone)]
pub struct CurrentRecipe {
    pub id: Uuid,
    pub recipe: Recipe,
}

impl Default for CurrentRecipe {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            recipe: Recipe::default(),
        }
    }
}

pub fn save_recipes(file: &PathBuf, recipes: &HashMap<Uuid, Recipe>) {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file)
        .expect("Couldn't open database file!");
    let writer = BufWriter::new(file);

    let pretty = PrettyConfig::new()
        .depth_limit(2)
        .separate_tuple_members(true)
        .enumerate_arrays(true);

    ron::ser::to_writer_pretty(writer, recipes, pretty).expect("Couldn't write to the database!");
}
