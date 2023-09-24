use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class="home-page">
            <h1>{"Welcome to my recipes app"}</h1>
            <form action="submit">
            <input type="text" placeholder="recipe name"/>
            <button>{"search recipes"}</button>
            </form>
        </div>
    }
}
