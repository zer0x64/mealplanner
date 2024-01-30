use std::{collections::HashMap, path::PathBuf};

use dioxus::prelude::*;
use uuid::Uuid;

use crate::{
    page::Page,
    recipe::{save_recipes, CurrentRecipe, Recipe},
};

#[component]
pub fn RecipeEditor(cx: Scope, file: PathBuf) -> Element {
    let page = use_shared_state::<Page>(cx).unwrap();
    let recipes_state = use_shared_state::<HashMap<Uuid, Recipe>>(cx).unwrap();
    let current_recipe_state = use_shared_state::<CurrentRecipe>(cx).unwrap();

    let name = use_state(cx, || current_recipe_state.read().recipe.name.clone());
    let ingredients = use_ref(cx, || {
        current_recipe_state.read().recipe.ingredients.clone()
    });

    cx.render(rsx! {
        div {
            h2 {
                "Name"
            }
            input {
                value: "{name}",
                oninput: move |event| {
                        name.set(event.value.clone());
                }
            }

            h2 {
                "Ingredients"
            }
            for (i, ing) in ingredients.read().iter().enumerate() {
                div {
                    input {
                        value: "{ing}",
                        oninput: move |event| {
                            ingredients.write()[i] = event.value.clone();
                        }
                    },
                    button {
                        onclick: move |_| {
                            ingredients.write().remove(i);
                        },
                        "x"
                    }
                }
            }
            div {
                button {
                    onclick: move |_| {
                        ingredients.write().push("".to_string());
                    },
                    "+"
                }
            }
            button {
                onclick: move |_| {
                    current_recipe_state.write().recipe.name = name.to_string();
                    current_recipe_state.write().recipe.ingredients = ingredients.read().clone();
                    recipes_state.write().insert(current_recipe_state.read().id.clone(), current_recipe_state.read().recipe.clone());

                    save_recipes(file, &recipes_state.read().clone());
                    *page.write() = Page::List;
                },
                "Save"
            }
        }
    })
}
