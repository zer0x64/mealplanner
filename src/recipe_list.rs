use std::{collections::HashMap, path::PathBuf};

use dioxus::prelude::*;
use uuid::Uuid;

use crate::{
    page::Page,
    recipe::{save_recipes, CurrentRecipe, Recipe},
};

#[component]
pub fn RecipeList(cx: Scope, file: PathBuf) -> Element {
    let recipes_state = use_shared_state::<HashMap<Uuid, Recipe>>(cx).unwrap();

    let recipes = recipes_state.read().clone();

    let page = use_shared_state::<Page>(cx).unwrap();
    let current_recipe = use_shared_state::<CurrentRecipe>(cx).unwrap();

    cx.render(rsx! {
        div {
            button {
                onclick: move |_| {
                    *current_recipe.write_silent() = CurrentRecipe::default();
                    *page.write() = Page::Editor;
                },
                "+"
            }
            for (k, r) in recipes {
                div {
                    button {
                        onclick: move |_| {
                            *current_recipe.write_silent() = CurrentRecipe {
                                id: k,
                                recipe: r.clone(),
                            };
                            *page.write() = Page::Editor;
                        },
                        r.name.clone()
                    }
                    button {
                        onclick: move |_| {
                            let mut recipes = recipes_state.write();
                            recipes.remove(&k);
                            save_recipes(file, &recipes)
                        },
                        "x"
                    }
                }
            }
        }
    })
}
