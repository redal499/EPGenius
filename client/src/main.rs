//main.rs
use dioxus::prelude::*;

use components::{Hero, Account, Interests, Generation, Tvgid, Navbar};

mod components;

#[derive(Clone, Copy)]
struct IsAuthorized(bool);


#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Hero {},
    #[route("/account")]
    Account {},
    #[route("/interests")]
    Interests {},
    #[route("/generation")]
    Generation {},
    #[route("/tvgid")]
    Tvgid {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    use_context_provider(|| Signal::new(IsAuthorized(false)));
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<Route> {}
    }
}