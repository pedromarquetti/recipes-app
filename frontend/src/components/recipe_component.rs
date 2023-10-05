use db::structs::{FullRecipe, Recipe};
use yew::prelude::*;

use super::{ingredient_list_component::IngredientList, steps_component::StepsList};

#[derive(Properties, PartialEq)]
pub struct RecipeProps {
    pub recipe: FullRecipe,
}
#[function_component(RecipeComponent)]
pub fn recipe_component(RecipeProps { recipe }: &RecipeProps) -> Html {
    html! {
    <div class="recipe">
        <h1 class="title">{recipe.recipe.recipe_name.clone()}</h1>
        <IngredientList ingredients={recipe.recipe.recipe_ingredients.clone()}/>
        <StepsList steps={recipe.steps.clone()}/>


    </div>
    }
}
