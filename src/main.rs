#![allow(non_snake_case)]
use dioxus::prelude::*;
use tracing::Level;

mod routes;
mod components;

use routes::Route;
// use components::{Header, Footer, Home, About, Product, Location, Login, Prepare, Desc};

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}