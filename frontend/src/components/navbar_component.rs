use crate::views::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(NavBar)]
pub fn navbar() -> Html {
    html! {
        <nav class="top-nav">
            <Link<Route> to={Route::Home} >{"Home"}</ Link<Route>>
            <Link<Route> to={Route::NewRecipe}>{"New Recipe"}</Link<Route>>
            <Link<Route> to={Route::UserPage}>{"User Register/Login"}</Link<Route>>

        </nav>
    }
}
