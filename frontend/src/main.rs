use crate::pages::{switch, Route};

use yew::{prelude::*, Renderer};
use yew_router::prelude::*;

pub mod components;
pub mod pages;

/// Main App function
#[function_component]
fn App() -> Html {
    html! {
        <BrowserRouter>
        <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    // wasm_logger::init(wasm_logger::Config::new(log::Level::Info));
    wasm_logger::init(wasm_logger::Config::default());

    Renderer::<App>::new().render();
}
