use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Clone)]
pub struct Recipe {
    pub name: String,
    pub ingredients: Vec<String>,
}
