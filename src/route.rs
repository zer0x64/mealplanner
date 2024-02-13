use dioxus::prelude::*;
use dioxus_router::prelude::*;
use uuid::Uuid;

use crate::cli::Cli;
use crate::config::{save_config, Config, Theme};
use crate::recipe_choser::RecipeChoser;
use crate::recipe_editor::RecipeEditor;
use crate::recipe_list::RecipeList;

#[derive(Routable, Clone)]
pub enum Route {
    #[layout(NavBar)]
    #[route("/")]
    #[route("/choser")]
    RecipeChoser {},

    #[route("/list")]
    RecipeList {},

    #[route("/editor/:id")]
    RecipeEditor { id: Uuid },
    #[end_layout]
    #[route("/navbar")]
    NavBar {},
}

#[component]
fn NavBar(cx: Scope) -> Element {
    let cli = use_shared_state::<Cli>(cx).unwrap();

    // Set theme
    let config_state = use_shared_state::<Config>(cx).unwrap();
    let theme = config_state.read().theme.clone();

    let create_eval = use_eval(cx);

    let eval = create_eval(
        r#"
        let theme = await dioxus.recv();

        if (theme == "dark" || (theme == "system" && window.matchMedia('(prefers-color-scheme: dark)').matches)) {
            document.documentElement.classList.add('dark')
        } else {
            document.documentElement.classList.remove('dark')
        };
        "#,
    ).unwrap();

    eval.send(theme.to_string().into()).unwrap();

    render! {
        div {
            nav {
                class: "mb-4",
                ul {
                    class: "flex",
                    li {
                        class: "mr-4",
                        Link {
                            to: Route::RecipeChoser {},
                            class: "text-blue-500 hover:text-blue-700",
                            "Choser"
                        }
                    }
                    li {
                        class: "mr-4",
                        Link {
                            to: Route::RecipeList {},
                            class: "text-blue-500 hover:text-blue-700",
                            "List"
                        }
                    }
                    li {
                        class: "mr-4 ml-auto",
                        button {
                            class: "ml-2 px-4 py-2 bg-white text-black dark:bg-black dark:text-white rounded",
                            onclick: move |_| {
                                let theme = match theme {
                                    Theme::System => Theme::Light,
                                    Theme::Light => Theme::Dark,
                                    Theme::Dark => Theme::System,
                                };

                                eval.send(theme.to_string().into()).unwrap();

                                config_state.write().theme = theme;

                                save_config(&cli.read().file, &config_state.read());
                            },
                            "Toggle Dark Theme"
                        }
                    }
                }
            }
            Outlet::<Route> {}
        }
    }
}
