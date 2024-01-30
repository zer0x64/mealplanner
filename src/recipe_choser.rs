use std::collections::HashMap;

use dioxus::prelude::*;
use rand::seq::IteratorRandom;
use uuid::Uuid;

use crate::recipe::Recipe;

pub fn RecipeChoser(cx: Scope) -> Element {
    let recipes = use_shared_state::<HashMap<Uuid, Recipe>>(cx).unwrap();

    let recipes_len = recipes.read().len();

    let chosen_recipe: &UseState<Vec<Recipe>> =
        use_state(cx, || chose_random_recipe(recipes.read().values(), 3));

    let number_recipe: &UseState<usize> = use_state(cx, || 3);

    cx.render(rsx! {
        div {
            input {
                "type": "number",
                "min": "0",
                "max": "{recipes_len}",
                value: "{number_recipe}",
                oninput: move |event| {
                    if let Ok(val) = event.value.parse() {
                        number_recipe.set(std::cmp::min(val, recipes_len));
                    };
                }
            }
            button {
                onclick: move |_| chosen_recipe.set(chose_random_recipe(recipes.read().values(), *number_recipe.get())),
                "Pick!"
            }
            for r in chosen_recipe.iter().map(Recipe::clone) {
                div {
                    b {r.name}
                    for i in r.ingredients {
                        div {i}
                    }
                    br {}
                }
            }
        }
    })
}

fn chose_random_recipe<'a>(
    recipe: impl IntoIterator<Item = &'a Recipe>,
    num: usize,
) -> Vec<Recipe> {
    recipe
        .into_iter()
        .choose_multiple(&mut rand::thread_rng(), num)
        .into_iter()
        .map(Recipe::clone)
        .collect()
}
