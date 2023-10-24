use db::structs::FullRecipe;
use yew::prelude::*;

use crate::components::recipe_title::RecipeTitle;

use super::{ingredient_list_component::IngredientList, steps_component::StepsList};

#[derive(Properties, PartialEq)]
pub struct RecipeProps {
    pub recipe: FullRecipe,
}
#[function_component(RecipeComponent)]
pub fn recipe_component(RecipeProps { recipe }: &RecipeProps) -> Html {
    html! {
    <div class="recipe">
        <RecipeTitle recipe={recipe.recipe.clone()}/>
        <IngredientList ingredients={
            recipe.ingredients.clone()
            }/>
        <StepsList steps={recipe.steps.clone()}/>


    </div>
    }
}
