//hero.rs
use dioxus::prelude::*;
use crate::Route;
use crate::IsAuthorized;

const HERO_CSS: Asset = asset!("/assets/styling/hero.css");

#[component]
pub fn Hero() -> Element {
    let isAuthorized = use_context::<Signal<IsAuthorized>>();

    rsx! {
        document::Link { rel: "stylesheet", href: HERO_CSS }

        div { id: "hero",
            h1 {
                span { class: "highlight", "EPG" }
                "enius - твой персональный гид в мире "
                span { class: "highlight", "IPTV" }
            }
            p { class: "subtitle",
                "Не знаете, какой фильм или сериал посмотреть в ближайшее время? Перебираете каналы впустую?"
            }
            p { class: "description",
                "Наш сервис - это ваш персональный телегид, который поможет вам сэкономить время и насладиться просмотром любимых передач и фильмов"
            }

            div { class: "steps-container",
                div { class: "step",
                     img { class: "step-icon", src: "/assets/images/epg.png" ,alt:"epg icon"}
                    p { class: "step-text",
                    "Сбор данных EPG: Мы автоматически загружаем телепрограмму."
                    }
                }
                div { class: "step",
                     img { class: "step-icon", src: "/assets/images/ai.png",alt:"ai icon" }
                    p { class: "step-text",
                        "Анализ с помощью нейросети: Наш AI анализирует контент, включая описание и жанры."
                    }
                }
                div { class: "step",
                     img {class: "step-icon", src:"/assets/images/recommend.png", alt:"recommend icon"}
                    p { class: "step-text",
                    "Персональные рекомендации: Вы получаете список передач, отобранных специально для вас."
                   }
                }
            }
        }
        
        div { id: "hero",
        button { class: "try-button", 
                onclick: 
                if isAuthorized().0 {
                    move |_| {
                        use_navigator().push(Route::Generation {  });
                    }
                } else {
                    move |_| {
                        use_navigator().push(Route::Account {  });
                    }
                },
                "Попробуйте сами!" }
        }
    }
}