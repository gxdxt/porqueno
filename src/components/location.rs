use dioxus::prelude::*;
use crate::{components::{header::Header, footer::Footer}};

#[component]
pub fn Location() -> Element {
    rsx! {
        style { {include_str!("../../assets/main.css")} }
        Header {}
        div { class: "location-info", "Location Information Here" }
        Footer {}
    }
}