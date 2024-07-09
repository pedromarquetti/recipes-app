use crate::{
    components::{
        ingredient_component::IngredientItem, new_ingredient::NewIngredients, new_step::NewSteps,
        RecipePartProps,
    },
    functions::{recipe_functions::delete_recipe, ApiResponse},
    views::Route,
    DEFAULT_NOTIFICATION_DURATION,
};
use db::structs::{FullRecipe, Ingredient};
use log::{debug, error, info};
use yew::{platform::spawn_local, prelude::*};
use yew_notifications::{use_notification, Notification};
use yew_router::hooks::use_navigator;

#[derive(Properties, PartialEq, Clone)]
pub struct IngredientsProps {
    pub old_ingredients: Vec<Ingredient>,
    pub callback: Callback<Ingredient>,
    pub recipe_id: i32,
}

#[function_component(EditIngredient)]
pub fn edit_ingredients(props: &IngredientsProps) -> Html {
    let IngredientsProps {
        old_ingredients,
        callback,
        recipe_id,
    } = props;
    let add_ingredient_state = use_state(|| false);
    let old_ingredients = old_ingredients.clone();
    let ingredient_state = use_state(|| old_ingredients.clone());

    html! {
    <>
    <h1>{"Edit Ingredients"}</h1>


    // conditionally redering NewIngredients
    {if !*add_ingredient_state.clone() {
        html!{<button onclick={{let add_ingredient_state = add_ingredient_state.clone();Callback::from(move|_|{add_ingredient_state.set(true)})}}>{"Add new Ingredients"}</button>}

    } else {
        html!{}
    }}
    {if *add_ingredient_state.clone() {
        html!{<>
            <NewIngredients
            recipe_id={recipe_id}
            {callback}

            />
            <button onclick={{let add_ingredient_state = add_ingredient_state.clone(); Callback::from(move|_| {add_ingredient_state.set(false)})}}>{"Cancel add new ingredients"}</button>
            </>

        }
    }
    else{html!{}}}
    </>
    }
}
