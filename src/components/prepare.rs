use dioxus::prelude::*;

#[component]
pub fn Prepare() -> Element {
    rsx! {
        div { class: "preparing-screen",
            img {
                style: "width: 453px; height: 832px",
                src: "pqn_angele.png",
            }
        }
    }
}