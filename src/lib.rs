use dioxus::prelude::*;

#[derive(Routable, Clone)]
pub enum Route {
    #[route("/")]
    Home,

    #[route("/page")]
    Page,
}

#[component]
fn Home() -> Element {
    rsx! { "Hello World" }
}

#[component]
fn Page() -> Element {
    rsx! { "Hello Page" }
}
