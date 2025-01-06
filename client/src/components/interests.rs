use dioxus::prelude::*;
use crate::Route;

const INTERESTS_CSS: Asset = asset!("/assets/styling/interests.css");
const GENRE_IMAGE: Asset = asset!("/assets/images/genre.png");

#[component]
pub fn Interests() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: INTERESTS_CSS }

        div { class: "interests-container",
            div { class: "section",
                h2 { "Выберите жанры" }
                div { class: "genre-buttons",
                    div { class: "genre-button",
                        img { src: GENRE_IMAGE, alt: "Драма", class: "genre-image" }
                        span { "Драма" }
                    }
                    div { class: "genre-button",
                        img { src: GENRE_IMAGE, alt: "Комедия", class: "genre-image" }
                        span { "Комедия" }
                    }
                    div { class: "genre-button",
                        img { src: GENRE_IMAGE, alt: "Фантастика", class: "genre-image" }
                        span { "Фантастика" }
                    }
                    div { class: "genre-button",
                        img { src: GENRE_IMAGE, alt: "Ужасы", class: "genre-image" }
                        span { "Ужасы" }
                    }
                    div { class: "genre-button",
                        img { src: GENRE_IMAGE, alt: "Боевик", class: "genre-image" }
                        span { "Боевик" }
                    }
                    div { class: "genre-button",
                        img { src: GENRE_IMAGE, alt: "Романтика", class: "genre-image" }
                        span { "Романтика" }
                    }
                }
            }
            div { class: "section",
                h2 { "Длительность" }
                div { class: "range-slider",
                    input { r#type: "range", min: "10", max: "240", value: "80.5", class: "range-min" }
                    div { class: "range-labels",
                        span { "10 мин" }
                        span { "240 мин" }
                    }
                }
            }
            div { class: "section",
                h2 { "Персональные предпочтения" }
                div { class: "preferences",
                    input { r#type: "text", placeholder: "Введите ключевое слово", class: "preference-input" }
                    button { class: "add-button", "Добавить" }
                }
                ul { class: "preference-list",
                    li { class: "preference-item",
                        span { "Новый год" }
                        button { class: "remove-button", "✖" }
                    }
                    li { class: "preference-item",
                        span { "Страна Россия" }
                        button { class: "remove-button", "✖" }
                    }
                }
            }
            div { class: "section",
                h2 { "Сохранённые интересы" }
                ul { class: "saved-interests",
                    li { class: "saved-interest",
                        span { "Новогодняя русская комедия" }
                        button { class: "remove-button", "✖" }
                    }
                }
            }
            div { class: "buttons",
                button { class: "save-button", "Сохранить интересы" }
                button { class: "generate-button", 
                onclick: move |_| {
                    use_navigator().push(Route::Generation {  });
                },
                "Перейти к генерации" }
            }
        }
    }
}
