use dioxus::prelude::*;
use crate::components::{home::Home, about::About, product::Product, location::Location, login::Login};

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/about")]
    About {},
    #[route("/product/:id")]
    Product { id: i32 },
    #[route("/location")]
    Location {},
    #[route("/login")]
    Login {},
}