use crate::pages::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(GoHome)]
pub fn go_home() -> Html {
    html! {
        <Link<Route> to={Route::Home} >{"Home"}</ Link<Route>>
    }
}
