use dioxus::prelude::*;
use crate::{components::{header::Header, footer::Footer}};

#[component]
pub fn Login() -> Element {
    rsx! {
        style { {include_str!("../../assets/main.css")} }
        Header {}
        div { class: "login-form", "Login Page Content Here" }
        Footer {}
    }
}