#![allow(non_snake_case)]

mod cli;
mod config;
mod recipe;
mod recipe_choser;
mod recipe_editor;
mod recipe_list;
mod route;

use std::{fs::File, io::BufReader};

use clap::Parser;
use config::Config;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

use cli::Cli;
use route::Route;

fn main() {
    #[cfg(target_family = "wasm")]
    dioxus_web::launch(App);

    #[cfg(any(windows, unix))]
    dioxus_desktop::launch_cfg(
        App,
        dioxus_desktop::Config::new()
            .with_custom_index(format!(r#"
            <!DOCTYPE html>
            <html>
                <head>
                    <title>mealplanner</title>
                    <meta name="viewport" content="width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=no" />
                    <style>{}</style>
                    <!-- CUSTOM HEAD -->
                </head>
                <body class="max-w-4xl mx-auto p-4 font-sans bg-gray-100 dark:bg-slate-800 dark:text-slate-400">
                    <div id="main"></div>
                    <!-- MODULE LOADER -->
                </body>
            </html>
            "#, include_str!("../public/tailwind.css")))
            .with_background_color((245, 245, 245, 255)),
    )
}

fn App(cx: Scope) -> Element {
    // Setup global state
    use_shared_state_provider(cx, || Cli::parse());

    let cli = use_shared_state::<Cli>(cx).unwrap();

    use_shared_state_provider(cx, || {
        let file = match File::open(&cli.read().file) {
            Ok(f) => f,
            Err(e) => {
                println!("Error while reading config file: {e}");
                return Config::default();
            }
        };

        match ron::de::from_reader(BufReader::new(file)) {
            Ok(r) => r,
            Err(e) => {
                println!("Error while parsing config: {e}");
                Config::default()
            }
        }
    });

    cx.render(rsx! {
        Router::<Route>{}
    })
}
