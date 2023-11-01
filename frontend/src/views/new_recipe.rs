use db::structs::{FullRecipe, Ingredient, Recipe};
use log::debug;
use web_sys::{HtmlElement, HtmlInputElement};

use yew::{prelude::*, virtual_dom::VNode};
use yew_router::prelude::*;

use crate::{
    components::input_component::{Input, InputType},
    views::Route,
};

#[function_component(NewRecipe)]
pub fn new_recipe() -> Html {
    let recipe_state = use_state(|| FullRecipe::new());

    let recipe_name_ref = use_node_ref();
    let recipe_obs_ref = use_node_ref();

    // on submit handler
    let onsubmit = {
        let recipe_name = recipe_name_ref.clone();
        let recipe_observations = recipe_obs_ref.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let name = recipe_name.cast::<HtmlInputElement>().unwrap().value();
            let obs = recipe_observations
                .cast::<HtmlInputElement>()
                .unwrap()
                .value();
            debug!("name {:?} obs {:?}", name, obs);
        })
    };

    html! {
    <>
        <h1>{"New Recipe"}</h1>
        <form {onsubmit} class="new-recipe">
            <Input
                input_node_ref={recipe_name_ref.clone()}
                input_placeholder="Recipe name"
                input_name="recipe name"
                input_type={InputType::Text}/>
            <Input
                input_node_ref={recipe_obs_ref.clone()}
                input_placeholder="Observations"
                input_name="observations"
                input_type={InputType::Text}/>
            <button >{"Create recipe"}</button>
        </form>
        <p>{"todo> make user create recipe first, then add ingredients/steps, because Step and Ingredient structs require recipe_id"}</p>
        <IngredientList/>
    </>
    }
}

#[function_component(IngredientList)]
pub fn ingredient_list() -> Html {
    let ingredient_list_state: UseStateHandle<Vec<Ingredient>> = use_state(|| vec![]);
    let name_input = use_node_ref();
    let ingredient_quantity_input = use_node_ref();
    let quantity_unit_input = use_node_ref();

    let onsubmit = {
        // cloning node ref
        let name_input = name_input.clone();
        let quantity_input = ingredient_quantity_input.clone();
        let unit_input = quantity_unit_input.clone();

        // cloning use_state
        let ingredient_list_state = ingredient_list_state.clone();
        Callback::from(move |event: SubmitEvent| {
            // they have to be cloned because of the 'move' inside the closure

            // "cloned" represents the "ingredient_list_state" vec![]
            // it'll be used to push new values
            let mut cloned = ingredient_list_state.to_vec();

            // getting form input values...
            let name = name_input.cast::<HtmlInputElement>().unwrap();
            let quantity = quantity_input.cast::<HtmlInputElement>().unwrap();
            let unit = unit_input.cast::<HtmlInputElement>().unwrap();

            event.prevent_default();

            // appending values to cloned vec![]
            cloned.push(Ingredient {
                id: None,
                recipe_id: 1,
                ingredient_name: name.value(),
                ingredient_quantity: quantity.value().parse::<i32>().unwrap(),
                quantity_unit: unit.value(),
            });
            // setting cloned local vec as the current list_state
            ingredient_list_state.set(cloned);
        })
    };
    let i: Html = ingredient_list_state
        .iter()
        .map(|ingredient: &Ingredient| {
            html! {
            <li>{ingredient.ingredient_name.clone()}</li>
            }
        })
        .collect::<VNode>();

    html! {
    <div class="new-ingredients">
        <form {onsubmit}>
            <Input
                input_node_ref={name_input.clone()}
                input_placeholder="Ingredient name"
                input_name="ingredient name"
                input_type={InputType::Text}/>
            <Input
            input_node_ref={ingredient_quantity_input.clone()}
            input_placeholder="Ingredient quantity"
            input_name="ingredient quantity"
            input_type={InputType::Text}/>
            <Input
                input_node_ref={quantity_unit_input.clone()}
                input_placeholder="Ingredient unit (kg,g,L)"
                input_name="ingredient unit"
                input_type={InputType::Text}/>

        </form>
        <ul>
        {i}
        </ul>
    </div>
    }
}
