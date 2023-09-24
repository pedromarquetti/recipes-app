use crate::components::go_home::GoHome;
use yew::prelude::*;

#[function_component(NavBar)]
pub fn navbar() -> Html {
    html! {
        <nav class="main-nav">
        <GoHome/>

        <p>{"todo: implement user login"}</p>
        </nav>
    }
}
