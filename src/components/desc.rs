use dioxus::prelude::*;

#[component]
pub fn Desc() -> Element {
    let porque_desc_kr1 = "
    뽀르께노는 2021년 여름 성북구 작은 골목의 열 다섯평 남짓한 비스트로를 시작으로 마포구 홍대의 타파스바(현재는 폐업), 동작구 이수의 카페앤펍을 운영하며 스페인의 매력을 서울 곳곳에서 알리고 있습니다. 
    스페인 세비야 바리오 산타크루즈의 좁은 골목 안 언제고 동네사람들로 북적이는 타파스바에서 영감을 받아, 서울에서 가장 접근성 있는 스페니쉬를 지향하며 남녀노소 누구나 즐길 수 있는 스페인의 맛과 멋을 추구합니다.
    ";

    let porque_desc_kr2 = "
    늘 “왜 안되겠어?” 라는 질문으로 무장된 마음으로 임하는 우리의 프로젝트는 거침이 없습니다. 
    우리에게 유한한 시간과 기회의 소중함을 알기에, 스페인과 관련된 키워드에서 뿐만 아니라 팀원들의 역량이 빛날 수 있는 영역이라면 주저없이 두드리고, 배워나가는 열린 마음으로 끊임없는 발전과 확장을 꿈꾸고자 합니다. 
    어떤 분야의 협업이든 긍정적으로 검토하겠습니다.
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
                        },
                        li { id: "carousel__slide2", tabindex: "0", class: "carousel__slide",
                            div { class: "carousel__snapper",
                                img {
                                    style: "width: 350px; height: 450px",
                                    src: "porqueno_table.jpg",
                                    class: "body_bg",
                                }
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