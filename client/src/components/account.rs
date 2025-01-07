//account.rs
use crate::IsAuthorized;
use dioxus::prelude::*;

const ACCOUNT_CSS: Asset = asset!("assets/styling/account.css");

#[component]
pub fn Account() -> Element {
    let mut isAuthorized = use_context::<Signal<IsAuthorized>>();
    let mut isRegistered = use_signal(|| false);

    rsx! {
        document::Link { rel: "stylesheet", href: ACCOUNT_CSS }
        
        if isAuthorized().0 {
            div { class: "account-management",
                h2 { class: "section-title", "Управление аккаунтом" }
                div { class: "change-password",
                    h3 { class: "subsection-title", "Сменить пароль:" }
                    input { class: "input-field", placeholder: "Старый пароль" }
                    input { class: "input-field", placeholder: "Новый пароль" }
                    button { class: "action-button", "Сменить пароль" }
                }
                button { 
                    class: "action-button delete-account",
                    onclick: move |_| {
                        isAuthorized.write().0 = false;
                    },
                    "Удалить аккаунт" 
                }
                div { class: "developer-contacts",
                    h3 { class: "subsection-title", "Контакты разработчика" }
                    p { "Email: developer@example.com" }
                }
                a { class: "terms-link", href: "#", "Пользовательское соглашение" }
            }
        } else {
            div { class: "auth-container",
                div { class: "auth-tabs",
                    button { 
                        class: "tab-button",
                        onclick: move |_| isRegistered.set(true), 
                        "Регистрация" 
                    }
                    button { 
                        class: "tab-button",
                        onclick: move |_| isRegistered.set(false), 
                        "Вход" 
                    }
                }
                if isRegistered() {
                    div { class: "auth-form register-form",
                        input { class: "input-field", placeholder: "Имя" }
                        input { class: "input-field", placeholder: "Дата рождения (опционально)" }
                        input { class: "input-field", placeholder: "E-mail" }
                        input { class: "input-field", placeholder: "Пароль", r#type: "password" }
                        button { 
                            class: "action-button",
                            onclick: move |_| {
                                let is_auth = true;
                                isAuthorized.write().0 = is_auth;
                            }, 
                            "Зарегистрироваться" 
                        }
                    }
                } else {
                    div { class: "auth-form login-form",
                        input { class: "input-field", placeholder: "E-mail" }
                        input { class: "input-field", placeholder: "Пароль", r#type: "password" }
                        button { 
                            class: "action-button",
                            onclick: move |_| {
                                let is_auth = true;
                                isAuthorized.write().0 = is_auth;
                            }, 
                            "Войти" 
                        }

                    }
                }
            }
        }
    }
}
