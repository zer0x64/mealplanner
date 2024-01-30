#![allow(non_snake_case)]

mod page;
mod recipe;
mod recipe_choser;
mod recipe_editor;
mod recipe_list;

use std::{collections::HashMap, fs::File, io::BufReader, path::PathBuf};

use clap::Parser;
use dioxus::prelude::*;
use uuid::Uuid;

use page::Page;
use recipe::{CurrentRecipe, Recipe};
use recipe_choser::RecipeChoser;
use recipe_editor::RecipeEditor;
use recipe_list::RecipeList;

#[derive(Debug, Parser)]
struct Cli {
    #[arg(default_value = "./recipes.ron")]
    file: PathBuf,
}

fn main() {
    dioxus_desktop::launch(App);
}

fn App(cx: Scope) -> Element {
    let cli = use_state(cx, || Cli::parse());

    // Setup global state
    use_shared_state_provider(cx, || {
        let file = match File::open(&cli.file) {
            Ok(f) => f,
            Err(_) => return HashMap::<Uuid, Recipe>::new(),
        };

        match ron::de::from_reader(BufReader::new(file)) {
            Ok(r) => r,
            Err(_) => HashMap::new(),
        }
    });

    use_shared_state_provider(cx, || CurrentRecipe::default());

    use_shared_state_provider(cx, || Page::Choser);

    // Get the useful state
    let current_page = use_shared_state::<Page>(cx).unwrap();

    cx.render(rsx! {
        head {
            style { include_str!("./styles.css") }
        },
        nav {
            ul {
                li {
                    button {
                        onclick: move |_| *current_page.write() = Page::Choser,
                        "Choser"
                    }
                }
                li {
                    button {
                        onclick: move |_| *current_page.write() = Page::List,
                        "Recipe List"
                    }
                }
            }
        }

        match *current_page.read() {
            Page::Choser => {
                rsx!{
                    RecipeChoser {}
                }
            },
            Page::List => {
                rsx!{
                    RecipeList {
                        file: cli.file.clone()
                    }
                }
            }
            Page::Editor => {
                rsx!{
                    RecipeEditor {
                        file: cli.file.clone()
                    }
                }
            }
        }

    })
}
