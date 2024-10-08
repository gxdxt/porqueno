use dioxus::prelude::*;
use crate::routes::Route;
use crate::{components::{header::Header, footer::Footer, desc::Desc, about::About}};

#[component]
pub fn Home() -> Element {
    // let count = use_signal(|| 0);

    rsx! {
        About {}
    }
}