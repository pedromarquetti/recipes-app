use crate::{
    functions::{recipe_functions::delete_recipe, ApiResponse},
    views::Route,
    DEFAULT_NOTIFICATION_DURATION,
};
use db::structs::{FullRecipe, Ingredient};
use log::{error, info};
use yew::{platform::spawn_local, prelude::*};
use yew_notifications::{use_notification, Notification};
use yew_router::hooks::use_navigator;

#[derive(Properties, PartialEq, Clone)]
pub struct IngredientsProps {
    curr_ingredients: Vec<Ingredient>,
    edited_ingredients: Callback<Vec<Ingredient>>,
}

#[function_component(IngredientEditor)]
pub fn edit_ingredients(props: &IngredientsProps) -> Html {
    let IngredientsProps {
        curr_ingredients,
        edited_ingredients,
    } = props;

    html! {}
}
