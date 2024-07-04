pub mod edit_ingredients;
pub mod edit_mode;
pub mod edit_steps;
pub mod ingredient_component;
pub mod input_component;
pub mod navbar_component;
pub mod new_ingredient;
pub mod new_step;
pub mod recipe_card_component;
pub mod recipe_component;
pub mod recipe_title;
pub mod steps_component;

use db::structs::RecipeTrait;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct RecipePartProps<T: PartialEq + RecipeTrait> {
    pub recipe_id: i32,
    #[prop_or_default]
    pub callback: Callback<T>,
}
