use std::env;

use crate::{
    components::navbar_component::NavBar,
    views::{switch, Route},
};

use yew::{prelude::*, Renderer};
use yew_router::prelude::*;

pub mod components;
pub mod functions;
pub mod views;

pub fn get_ip() -> String {
    env::var("API_IP").unwrap_or("http://127.0.0.1:3000".into())
}

/// Main App function
#[function_component]
fn App() -> Html {
    html! {

        <BrowserRouter>
            <NavBar />
            <div class="container">
            <Switch<Route>  render={switch} />
            </div>
        </BrowserRouter>
    }
}

fn main() {
    // wasm_logger::init(wasm_logger::Config::new(log::Level::Info));
    wasm_logger::init(wasm_logger::Config::default());

    Renderer::<App>::new().render();
}
