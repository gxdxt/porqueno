use dioxus::prelude::*;
use crate::{components::{header::Header, footer::Footer, desc::Desc}, routes::Route};

#[component]
pub fn About() -> Element {
    rsx! {
        style { {include_str!("../../assets/main.css")} }
        Header {}
        Desc {}
        Footer {}
    }
}