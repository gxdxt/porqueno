use dioxus::prelude::*;
use crate::routes::Route;

#[component]
pub fn Home() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        style { {include_str!("../../assets/main.css")} }
        div {
            class: "home_container",
            div {
                class: "home",
                Link {
                    to: Route::About {},
                    img {
                        src: "porqueno.png",
                        class: "logo",
                        height: "180px",
                        width: "240px",
                    }
                }
            }
        }
    }
}