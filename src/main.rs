use base_path::Route;
use dioxus::prelude::*;

fn main() {
    dioxus::logger::init(dioxus::logger::tracing::Level::TRACE).expect("logger failed to init");

    launch(|| {
        rsx! {
            Router::<Route> {}
        }
    });
}
