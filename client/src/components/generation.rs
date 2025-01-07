//generation.rs
use dioxus::prelude::*;

const GENERATION_CSS: Asset = asset!("/assets/styling/generation.css");

#[component]
pub fn Generation() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: GENERATION_CSS }

        div { class: "generation-container",
            div { class: "section",
                h2 { "Сохранённые интересы" }
                ul { class: "saved-interests",
                    li { class: "saved-interest",
                        span { "Новогодняя русская комедия" }
                        button { class: "remove-button", "✖" }
                    }
                }
            }
            div { class: "section",
                h2 { "Выберите период" }
                div { class: "date-inputs",
                    input { r#type: "date", class: "date-input" }
                    input { r#type: "date", class: "date-input" }
                }
            }
            button { class: "generate-button", "Начать генерацию" }
            div { class: "section",
                h2 { "Рекомендации" }
                table { class: "recommendations-table",
                    thead {
                        tr { 
                            th { "Название канала" }
                            th { "Название программы" }
                            th { "Время начала" }
                            th { "Время окончания" }
                            th { "Описание" }
                        }
                    }
                    tbody {
                        tr {
                            td { "Россия 1 HD" }
                            td { "Дела семейные" }
                            td { "04:10 01.10.2024" }
                            td { "05:57 01.10.2024" }
                            td { "Семья Соколовых – семья военных, празднует новоселье, и с этим событием каждый связывает самые радужные надежды" }
                        }
                        tr {
                            td { "Viju TV 1000 Русское HD" }
                            td { "Елки лохматые" }
                            td { "16:50 01.10.2024" }
                            td { "18:34 01.10.2024" }
                            td { "Фильм расскажет о двух собаках, Пирате и Йоко, которые проявят всю свою смекалку, чтобы защитить дом хозяев от воров." }
                        }
                    }
                }
            }
        }
    }
}
