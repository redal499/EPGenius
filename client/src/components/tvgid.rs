//tvgid.rs
use dioxus::prelude::*;

const TVGID_CSS: Asset = asset!("/assets/styling/tvgid.css");

#[component]
pub fn Tvgid() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: TVGID_CSS }

        div { class: "tvgid-container",
            div { class: "section",
                h2 { "Телепрограмма" }
                table { class: "tvgid-table",
                    thead {
                        tr {
                            th { "Название канала" }
                            th { "Время начала" }
                            th { "Время окончания" }
                            th { "Название передачи" }
                        }
                    }
                    tbody {
                        tr { class: "tvgid-row",
                        td { "Первый канал" }
                        td { "10:15 15.10.2024" }
                        td { "11:30 15.10.2024" }
                        td { "Доброе утро" }
                    }
                    tr { class: "tvgid-row",
                        td { "Россия 1" }
                        td { "10:30 15.10.2024" }
                        td { "11:45 15.10.2024" }
                        td { "Вести" }
                    }
                    tr { class: "tvgid-row",
                        td { "НТВ" }
                        td { "10:45 15.10.2024" }
                        td { "11:50 15.10.2024" }
                        td { "Сегодня" }
                    }
                    tr { class: "tvgid-row",
                        td { "ТНТ" }
                        td { "10:20 15.10.2024" }
                        td { "11:35 15.10.2024" }
                        td { "Comedy Club" }
                    }
                    tr { class: "tvgid-row",
                        td { "СТС" }
                        td { "10:10 15.10.2024" }
                        td { "11:25 15.10.2024" }
                        td { "Уральские пельмени" }
                    }
                    tr { class: "tvgid-row",
                        td { "Рен-ТВ" }
                        td { "10:40 15.10.2024" }
                        td { "11:55 15.10.2024" }
                        td { "Военная тайна" }
                    }
                    tr { class: "tvgid-row",
                        td { "ТВ-3" }
                        td { "10:25 15.10.2024" }
                        td { "11:40 15.10.2024" }
                        td { "Гадалка" }
                    }
                    tr { class: "tvgid-row",
                        td { "Пятница" }
                        td { "10:35 15.10.2024" }
                        td { "11:50 15.10.2024" }
                        td { "Орел и решка" }
                    }
                    tr { class: "tvgid-row",
                        td { "ТВЦ" }
                        td { "10:05 15.10.2024" }
                        td { "11:20 15.10.2024" }
                        td { "Настроение" }
                    }
                    tr { class: "tvgid-row",
                        td { "Звезда" }
                        td { "10:50 15.10.2024" }
                        td { "11:55 15.10.2024" }
                        td { "Служу России" }
                    }
                    tr { class: "tvgid-row",
                        td { "Культура" }
                        td { "10:15 15.10.2024" }
                        td { "11:30 15.10.2024" }
                        td { "Новости культуры" }
                    }
                    tr { class: "tvgid-row",
                        td { "Домашний" }
                        td { "10:40 15.10.2024" }
                        td { "11:45 15.10.2024" }
                        td { "Давай разведемся!" }
                    }
                    tr { class: "tvgid-row",
                        td { "Карусель" }
                        td { "10:20 15.10.2024" }
                        td { "11:35 15.10.2024" }
                        td { "Смешарики" }
                    }
                    tr { class: "tvgid-row",
                        td { "Спас" }
                        td { "10:30 15.10.2024" }
                        td { "11:40 15.10.2024" }
                        td { "Утро на Спасе" }
                    }
                    tr { class: "tvgid-row",
                        td { "Мир" }
                        td { "10:25 15.10.2024" }
                        td { "11:50 15.10.2024" }
                        td { "В гостях у цифры" }
                    }
                    tr { class: "tvgid-row",
                        td { "Муз-ТВ" }
                        td { "10:10 15.10.2024" }
                        td { "11:25 15.10.2024" }
                        td { "Русский чарт" }
                    }
                    tr { class: "tvgid-row",
                        td { "Че" }
                        td { "10:45 15.10.2024" }
                        td { "11:55 15.10.2024" }
                        td { "Улетное видео" }
                    }
                    tr { class: "tvgid-row",
                        td { "Ю" }
                        td { "10:35 15.10.2024" }
                        td { "11:45 15.10.2024" }
                        td { "Беременна в 16" }
                    }
                    tr { class: "tvgid-row",
                        td { "2x2" }
                        td { "10:05 15.10.2024" }
                        td { "11:20 15.10.2024" }
                        td { "Симпсоны" }
                    }
                    tr { class: "tvgid-row",
                        td { "Disney" }
                        td { "10:50 15.10.2024" }
                        td { "11:55 15.10.2024" }
                        td { "Утиные истории" }
                    }

                 
                    }
                }
            }
        }
    }
}
