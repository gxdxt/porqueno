#![allow(non_snake_case)]
use dioxus::prelude::*;
use tracing::Level;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
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

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Header() -> Element {
    let mut count = use_signal(|| 0);

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
                    to: Route::About {
                        // id: count()
                    },
                    "About"
                },
                Link {
                    to: Route::Location {

                    },
                    "Product"
                },
                Link {
                    to: Route::Location {
                        // id: count()
                    },
                    "Location"
                },
                Link {
                    to: Route::Login {
                        // id: count()
                    },
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

#[component]
fn Footer() -> Element {
    // 필요 정보
    // 주소
    // 사업자 번호
    // 대표 이름

    rsx! {
        div {
            class: "footer",
            div {
                class: "items",
                div {
                    class: "footer-item",
                    "COMPANY: PQN F&B"
                }
                div {
                    class: "footer-item",
                    "OWNER : 안재석"
                }
                div {
                    class: "footer-item",
                    "LICENSE : 294-25-01096 / 221-21-77979"
                }
                div {
                    class: "footer-item",
                    "ADDRESS : 서울특별시 성북구 동소문로6길 4-21 1층 / 서울특별시 동작구 사당로 307-12 1층"
                }
                div {
                    class: "footer-item",
                    "TEL : 010-7712-4561"
                }
                // div {
                //     class: "footer-item",
                //     "FAX : XXX-XXX-XXXX"
                // }
                // div {
                //     class: "footer-item",
                //     "MALL-ORDER LICENSE : XXXX-XXXX-XXXX"
                // }
                div {
                    class: "footer-item, footer-item-copyright",
                    "COPYRIGHT © PQN F&B ALL RIGHTS RESERVED."
                    "HOSTING & Designed by TAE"
                }
            }
        }
    }
}

#[component]
fn Product(id: i32) -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        style { {include_str!("../assets/main.css")} }
        Header {}
        Prepare {}
        Footer {}
    }
}

#[component]
fn Location() -> Element {
    rsx! {
        style { {include_str!("../assets/main.css")} }
        Header {}
        Prepare {}
        Footer {}
    }
}

#[component]
fn Login() -> Element {
    rsx! {
        style { {include_str!("../assets/main.css")} }
        Header {}
        Prepare {}
        Footer {}
    }
}

#[component]
fn About() -> Element {
    rsx! {
        style { {include_str!("../assets/main.css")} }
        Header {}
        Desc {}
        Footer {}
    }
}

#[component]
fn Prepare() -> Element {
    rsx! {
        div { class: "preparing-screen",
        img {
            style: "width: 453px; height: 832px",
            src: "pqn_angele.png",
    }
        }
    }
}

#[component]
fn Desc() -> Element {
    let porque_desc_kr1 = "
    뽀르께노는 2021년 여름 성북구 작은 골목의 열 다섯평 남짓한 비스트로를 시작으로 마포구 홍대의 타파스바(현재는 폐업), 동작구 이수의 카페앤펍을 운영하며 스페인의 매력을 서울 곳곳에서 알리고 있습니다. \n
    스페인 세비야 바리오 산타크루즈의 좁은 골목 안 언제고 동네사람들로 북적이는 타파스바에서 영감을 받아, 서울에서 가장 접근성 있는 스페니쉬를 지향하며 남녀노소 누구나 즐길 수 있는 스페인의 맛과 멋을 추구합니다. \n
    ";

    let porque_desc_kr2 = "
    늘 “왜 안되겠어?” 라는 질문으로 무장된 마음으로 임하는 우리의 프로젝트는 거침이 없습니다. \n
    우리에게 유한한 시간과 기회의 소중함을 알기에, \n 스페인과 관련된 키워드에서 뿐만 아니라 팀원들의 역량이 빛날 수 있는 영역이라면 주저없이 두드리고, 배워나가는 열린 마음으로 끊임없는 발전과 확장을 꿈꾸고자 합니다. 어떤 분야의 협업이든 긍정적으로 검토하겠습니다.    
    ";

    let porque_desc_es1 = "
    ¿Por qué no? comenzó en el verano de 2021 como un pequeño bistró de unos quince pyeong en un pequeño callejón de Seongbuk-gu, y ahora estamos llevando el encanto de España a varios rincones de Seúl, gestionando un bar de tapas en Hongdae, Mapo-gu (actualmente cerrado), y un café y pub en Isu, Dongjak-gu. \n
    Inspirados por un bar de tapas siempre concurrido por los vecinos en un estrecho callejón del Barrio de Santa Cruz en Sevilla, España, aspiramos a ofrecer la experiencia española más accesible en Seúl, buscando el sabor y el estilo de España que cualquiera, sin importar su edad o género, pueda disfrutar. \n
    ";

    let porque_desc_es2 = "
    Nuestro proyecto, siempre abordado con la pregunta “¿por qué no?”, no tiene límites. \n
    Somos conscientes del valor del tiempo y las oportunidades limitadas que tenemos, \n por lo que, no solo en lo relacionado con palabras clave de España, sino también en cualquier área donde brillen las habilidades de nuestro equipo, golpearemos sin dudarlo, aprendiendo con una mente abierta y buscando un desarrollo y expansión constantes. \n Consideraremos positivamente la colaboración en cualquier campo.
    ";

    rsx! {
        div {
            class: "desc",
                div {
                    section { class: "carousel", "aria-label": "Gallery",
            ol { class: "carousel__viewport",
                li { id: "carousel__slide1", tabindex: "0", class: "carousel__slide",
                    div { class: "carousel__snapper",
                    img {
                        style: "width: 350px; height: 450px",
                        src: "porqueno_facade.jpg",
                        class: "body_bg",
                }
                    }
                }
                li { id: "carousel__slide2", tabindex: "0", class: "carousel__slide",
                    div { class: "carousel__snapper",
                    img {
                        style: "width: 350px; height: 450px",
                        src: "porqueno_table.jpg",
                        class: "body_bg",
                }
                    }
                }
                li { id: "carousel__slide3", tabindex: "0", class: "carousel__slide",
                    div { class: "carousel__snapper" }
                }
                li { id: "carousel__slide4", tabindex: "0", class: "carousel__slide",
                    div { class: "carousel__snapper" }
                }
            }
            aside { class: "carousel__navigation",
                ol { class: "carousel__navigation-list",
                    li { class: "carousel__navigation-item",
                        a { href: "#carousel__slide1", class: "carousel__navigation-button", "Go to slide 1" }
                    }
                    li { class: "carousel__navigation-item",
                        a { href: "#carousel__slide2", class: "carousel__navigation-button", "Go to slide 2" }
                    }
                    li { class: "carousel__navigation-item",
                        a { href: "#carousel__slide3", class: "carousel__navigation-button", "Go to slide 3" }
                    }
                    li { class: "carousel__navigation-item",
                        a { href: "#carousel__slide4", class: "carousel__navigation-button", "Go to slide 4" }
                    }
                }
            }
        }

                    div {
                        class: "desc_detail1",
                        h1 { "Somos" },
                        "{porque_desc_kr1}"
                    }
                    img {
                        style: "width: 500px; height: 350px",
                        src: "porqueno.png",
                        class: "body_bg",
                }
                    div {
                        class: "desc_detail2",
                        h1 { "¿Por qué no?" }
                        "{porque_desc_kr2}"
                    }
                }
        }
    }
}

#[component]
fn Home() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
            style { {include_str!("../assets/main.css")} }
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
                // div {
                //     class: "home_menu",
                //     Link {
                //         to: Route::About {
                //             // id: count()
                //         },
                //         "About"
                //     },
                //     Link {
                //         to: Route::Product {
                //             id: count()
                //         },
                //         "Product"
                //     },
                //     Link {
                //         to: Route::Location {
                //             // id: count()
                //         },
                //         "Location"
                //     },
                //     Link {
                //         to: Route::Login {
                //             // id: count()
                //         },
                //         "Login"
                //     },
                // }
            //    div {
            //         hr { class: "divider", style: "height: 2px; background-color: #92000A;" }
            //         hr { class: "divider", style: "height: 2px;" }
            //         hr { class: "divider", style: "height: 2px; background-color: #92000A;" }
            //     }
            }
        }
    }
}
