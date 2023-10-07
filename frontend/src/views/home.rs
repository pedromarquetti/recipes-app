use log::{debug, info};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{
    components::{
        go_home_component::GoHome,
        input_component::{Input, InputType},
    },
    views::Route,
};

#[function_component(Home)]
pub fn home() -> Html {
    let recipe_search = use_node_ref();
    let navigator = use_navigator().unwrap();

    let onsubmit = {
        let recipe_search = recipe_search.clone();

        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            let search = recipe_search.cast::<HtmlInputElement>().unwrap().value();
            navigator.push(&Route::RecipeList { name: search })
        })
    };

    html! {
        <div class="home-page">
            <h1>{"Welcome to my recipes app"}</h1>
            <form {onsubmit}>
            <Input
            input_node_ref={recipe_search}
            input_placeholder="type search string"
            input_name="search"
            input_type={InputType::Text}
            />
            <button>{"search recipes"}</button>
            </form>
        </div>
    }
}
