use crate::components::go_home_component::GoHome;
use yew::prelude::*;

#[function_component(NavBar)]
pub fn navbar() -> Html {
    html! {
        <nav class="top-nav">
        <GoHome/>


        <p>{"todo: implement user login"}</p>
        </nav>
    }
}
