use std::collections::HashMap;

use dioxus::prelude::*;
use dioxus_router::hooks::use_navigator;
use uuid::Uuid;

use crate::{
    cli::Cli,
    recipe::{save_recipes, CurrentRecipe, Recipe},
};

#[component]
pub fn RecipeList(cx: Scope) -> Element {
    let recipes_state = use_shared_state::<HashMap<Uuid, Recipe>>(cx).unwrap();
    let cli = use_shared_state::<Cli>(cx).unwrap();

    let recipes = recipes_state.read().clone();

    let current_recipe = use_shared_state::<CurrentRecipe>(cx).unwrap();

    cx.render(rsx! {
        div {
            class: "flex flex-col space-y-4",
            div {
                class: "flex items-center space-x-2",
                button {
                    class: "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline",
                    onclick: move |_| {
                        *current_recipe.write_silent() = CurrentRecipe::default();
                        use_navigator(cx).push(crate::route::Route::RecipeEditor {});
                    },
                    "+"
                }    
            }

            for (k, r) in recipes {
                div {
                    class: "flex flex-row space-x-2",
                    button {
                        class: "bg-gray-500 hover:bg-gray-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline h-full",
                        onclick: move |_| {
                            *current_recipe.write_silent() = CurrentRecipe {
                                id: k,
                                recipe: r.clone(),
                            };
                            use_navigator(cx).push(crate::route::Route::RecipeEditor {});
                        },
                        r.name.clone()
                    }
                    button {
                        class: "bg-red-500 hover:bg-red-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline h-full",
                        onclick: move |_| {
                            let mut recipes = recipes_state.write();
                            recipes.remove(&k);
                            save_recipes(&cli.read().file, &recipes)
                        },
                        "x"
                    }
                }
            }
        }
        
    })
}
