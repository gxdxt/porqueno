use dioxus::prelude::*;
use crate::routes::Route;

#[component]
pub fn Header() -> Element {
    let count = use_signal(|| 0);

    rsx! {
        div {
            class: "topnav",
            Link {
                to: Route::Home {},
                img {
                    src: "porqueno.png",
                    class: "logo",
                    height: "180px",
                    width: "240px",
                }
            }
            div {
                class: "topnav-right",
                Link {
                    to: Route::About {},
                    "About"
                },
                Link {
                    to: Route::Location {},
                    "Product"
                },
                Link {
                    to: Route::Location {},
                    "Location"
                },
                Link {
                    to: Route::Login {},
                    "Login"
                },
            }
           div {
                class: "main_divider",
                hr { class: "divider", style: "height: 2px; background-color: #92000A;" }
                hr { class: "divider", style: "height: 2px;" }
                hr { class: "divider", style: "height: 2px; background-color: #92000A;" }
            }
        }
    }
}