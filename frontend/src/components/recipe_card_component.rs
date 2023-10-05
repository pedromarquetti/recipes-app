use db::structs::Recipe;
use yew::prelude::*;

use super::{ingredient_list_component::IngredientList, steps_component::StepsList};

#[derive(Properties, PartialEq)]
pub struct RecipeProps {
    pub recipe: Recipe,
}
#[function_component(RecipeCard)]
pub fn recipe_card(RecipeProps { recipe }: &RecipeProps) -> Html {
    html! {
    <div class="recipe">
        <h1 class="title">{recipe.recipe_name.clone()}</h1>
        <IngredientList ingredients={recipe.recipe_ingredients.clone()}/>
    </div>
    }
}
