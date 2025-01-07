//navbar.rs
use crate::Route;
use dioxus::prelude::*;
use crate::IsAuthorized;

const NAVBAR_CSS: Asset = asset!("/assets/styling/navbar.css");

#[component]
pub fn Navbar() -> Element {
    let isAuthorized = use_context::<Signal<IsAuthorized>>();
    rsx! {
        document::Link { rel: "stylesheet", href: NAVBAR_CSS }
        if isAuthorized().0 {
            div {
                id: "navbar",
                Link {
                    to: Route::Hero {},
                    "Главная"
                }
                Link {
                    to: Route::Generation {},
                    "Генерация контента"
                }
                Link {
                    to: Route::Interests{},
                    "Мои интересы"
                }
                Link {
                    to: Route::Tvgid {},
                    "ТВ-гид"
                }
                Link {
                    to: Route::Account {},
                    "Аккаунт"
                }
            }
        } else {
            div {
                id: "navbar",
                Link {
                    to: Route::Hero {},
                    "Главная"
                }
                Link {
                    to: Route::Account {},
                    "Генерация контента"
                }
                Link {
                    to: Route::Account {},
                    "Мои интересы"
                }
                Link {
                    to: Route::Tvgid {},
                    "ТВ-гид"
                }
                Link {
                    to: Route::Account {},
                    "Аккаунт"
                }
            }
        }
        
        Outlet::<Route> {}
    }
}
