use dioxus::prelude::*;
use dioxus_router::hooks::use_navigator;
use uuid::Uuid;

use crate::{
    cli::Cli,
    config::{save_config, Config},
};

#[component]
pub fn RecipeList(cx: Scope) -> Element {
    let config_state = use_shared_state::<Config>(cx).unwrap();
    let cli = use_shared_state::<Cli>(cx).unwrap();

    let recipes = config_state.read().recipes.clone();

    cx.render(rsx! {
        div {
            class: "flex flex-col space-y-4",
            div {
                class: "flex items-center space-x-2",
                button {
                    class: "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline",
                    onclick: move |_| {
                        use_navigator(cx).push(crate::route::Route::RecipeEditor {
                            id: Uuid::new_v4()
                        });
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
                            use_navigator(cx).push(crate::route::Route::RecipeEditor {
                                id: k
                            });
                        },
                        r.name.clone()
                    }
                    button {
                        class: "bg-red-500 hover:bg-red-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline h-full",
                        onclick: move |_| {
                            let mut config = config_state.write();
                            config.recipes.remove(&k);
                            save_config(&cli.read().file, &config)
                        },
                        "x"
                    }
                }
            }
        }
    })
}
