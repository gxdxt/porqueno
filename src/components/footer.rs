use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
        div {
            class: "footer",
            div {
                class: "items",
                div {
                    class: "footer-item",
                    "COMPANY: PQN F&B"
                },
                div {
                    class: "footer-item",
                    "OWNER : 안재석"
                },
                div {
                    class: "footer-item",
                    "LICENSE : 294-25-01096 / 221-21-77979"
                },
                div {
                    class: "footer-item",
                    "ADDRESS : 서울특별시 성북구 동소문로6길 4-21 1층 / 서울특별시 동작구 사당로 307-12 1층"
                },
                div {
                    class: "footer-item",
                    "TEL : 010-7712-4561"
                },
                div {
                    class: "footer-item, footer-item-copyright",
                    "COPYRIGHT © PQN F&B ALL RIGHTS RESERVED."
                    "HOSTING & Designed by TAE"
                }
            }
        }
    }
}