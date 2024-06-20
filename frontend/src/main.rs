pub mod components;
pub mod functions;
pub mod views;

use crate::{
    components::navbar_component::NavBar,
    views::{switch, Route},
};
use yew::{prelude::*, Renderer};
use yew_notifications::{
    Notification, NotificationFactory, NotificationsPosition, NotificationsProvider,
};
use yew_router::prelude::*;
/// notification Timeout in seconds
const DEFAULT_NOTIFICATION_DURATION: time::Duration = time::Duration::seconds(2);

/// Main App function
#[function_component]
fn App() -> Html {
    let component_creator = NotificationFactory;

    html! {
        <NotificationsProvider<Notification,NotificationFactory> {component_creator} position={NotificationsPosition::TopLeft} >

            <BrowserRouter>
                <NavBar />
                    <div class="container">
                        <Switch<Route>  render={switch} />
                    </div>
            </BrowserRouter>
        </NotificationsProvider<Notification,NotificationFactory>>

    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    Renderer::<App>::new().render();
}
