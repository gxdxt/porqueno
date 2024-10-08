use dioxus::prelude::*;
use crate::{components::{header::Header, footer::Footer}};

#[component]
pub fn Product(id: i32) -> Element {
    rsx! {
        style { {include_str!("../../assets/main.css")} }
        Header {}
        div { class: "product-info", "Product ID: {id}" }
        Footer {}
    }
}