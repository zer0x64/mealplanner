use std::{collections::HashMap, fs::OpenOptions, io::BufWriter, path::PathBuf};

use ron::ser::PrettyConfig;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::recipe::Recipe;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Clone)]
pub enum Theme {
    #[default]
    System,
    Light,
    Dark,
}

impl ToString for Theme {
    fn to_string(&self) -> String {
        match self {
            &Theme::System => "system".to_string(),
            &Theme::Light => "light".to_string(),
            &Theme::Dark => "dark".to_string(),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Clone)]
pub struct Config {
    pub theme: Theme,
    pub recipes: HashMap<Uuid, Recipe>,
}

pub fn save_config(file: &PathBuf, config: &Config) {
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

    ron::ser::to_writer_pretty(writer, config, pretty).expect("Couldn't write to the database!");
}
