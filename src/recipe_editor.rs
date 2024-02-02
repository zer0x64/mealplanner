use std::collections::HashMap;

use dioxus::prelude::*;
use dioxus_router::hooks::use_navigator;
use uuid::Uuid;

use crate::{
    cli::Cli,
    recipe::{save_recipes, CurrentRecipe, Recipe},
};

#[component]
pub fn RecipeEditor(cx: Scope) -> Element {
    let recipes_state = use_shared_state::<HashMap<Uuid, Recipe>>(cx).unwrap();
    let current_recipe_state = use_shared_state::<CurrentRecipe>(cx).unwrap();
    let cli = use_shared_state::<Cli>(cx).unwrap();

    let name = use_state(cx, || current_recipe_state.read().recipe.name.clone());
    let ingredients = use_ref(cx, || {
        current_recipe_state.read().recipe.ingredients.clone()
    });

    cx.render(rsx! {
        div {
            h2 {
                class: "text-xl font-semibold mb-2",
                "Name"
            }
            input {
                value: "{name}",
                class: "border rounded-md p-2 mb-4",
                oninput: move |event| {
                        name.set(event.value.clone());
                }
            }

            h2 {
                class: "text-xl font-semibold mb-2",
                "Ingredients"
            }
            for (i, ing) in ingredients.read().iter().enumerate() {
                div {
                    class: "mb-4",
                    input {
                        class: "border rounded-md p-2",
                        value: "{ing}",
                        oninput: move |event| {
                            ingredients.write()[i] = event.value.clone();
                        }
                    },
                    button {
                        class: "ml-2 px-4 py-2 bg-red-500 text-white rounded",
                        onclick: move |_| {
                            ingredients.write().remove(i);
                        },
                        "x"
                    }
                }
            }
            div {
                class: "mb-4",
                button {
                    class: "px-4 py-2 bg-blue-500 text-white rounded",
                    onclick: move |_| {
                        ingredients.write().push("".to_string());
                    },
                    "+"
                }
            }
            button {
                class: "px-4 py-2 bg-green-500 text-white rounded",
                onclick: move |_| {
                    current_recipe_state.write().recipe.name = name.to_string();
                    current_recipe_state.write().recipe.ingredients = ingredients.read().clone();
                    recipes_state.write().insert(current_recipe_state.read().id.clone(), current_recipe_state.read().recipe.clone());

                    save_recipes(&cli.read().file, &recipes_state.read().clone());

                    use_navigator(cx).push(crate::route::Route::RecipeList {});
                },
                "Save"
            }
        }
    })
}
