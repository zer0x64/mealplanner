use dioxus::prelude::*;
use dioxus_router::prelude::*;

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

    #[route("/editor/")]
    RecipeEditor {},
    #[end_layout]
    #[route("/test")]
    NavBar {},
}

#[component]
fn NavBar(cx: Scope) -> Element {
    render! {
        body {
            class: "max-w-4xl mx-auto p-4 font-sans bg-gray-100",
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
                }
            }
            Outlet::<Route> {}
        }
    }
}
