use dioxus::prelude::*;
use dioxus_router::hooks::use_navigator;
use uuid::Uuid;

use crate::{
    cli::Cli,
    config::{save_config, Config},
    recipe::Recipe,
};

#[component]
pub fn RecipeEditor(cx: Scope, id: Uuid) -> Element {
    let config_state = use_shared_state::<Config>(cx).unwrap();
    let mut current_recipe = config_state
        .read()
        .recipes
        .get(id)
        .unwrap_or(&mut Recipe::default())
        .clone();

    let cli = use_shared_state::<Cli>(cx).unwrap();

    let name = use_state(cx, || current_recipe.name.clone());
    let ingredients = use_ref(cx, || current_recipe.ingredients.clone());

    cx.render(rsx! {
        div {
            h2 {
                class: "text-xl font-semibold mb-2",
                "Name"
            }
            input {
                value: "{name}",
                class: "border rounded-md p-2 mb-4 dark:bg-slate-500 dark:text-white",
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
                        class: "border rounded-md p-2 dark:bg-slate-500 dark:text-white",
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
                    current_recipe.name = name.to_string();
                    current_recipe.ingredients = ingredients.read().clone();
                    config_state.write().recipes.insert(id.clone(), current_recipe.clone());

                    save_config(&cli.read().file, &config_state.read().clone());

                    use_navigator(cx).push(crate::route::Route::RecipeList {});
                },
                "Save"
            }
        }
    })
}
