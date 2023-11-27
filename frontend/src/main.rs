use std::env;

use crate::{
    components::navbar_component::NavBar,
    views::{switch, Route},
};

use yew::{prelude::*, Renderer};
use yew_router::prelude::*;

//yew_notifications is not Yew0.21 compatible yet
// use yew_notifications::{
//     Notification, NotificationFactory, NotificationsPosition, NotificationsProvider,
// };

pub mod components;
pub mod functions;
pub mod views;

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
