use super::{ingredient_list_component::IngredientList, steps_component::StepsList};
use crate::views::Route;
use db::structs::Recipe;
use yew::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct RecipeProps {
    pub recipe: Recipe,
}
#[function_component(RecipeCard)]
/// Simplified card representing Recipe, containing a button that redirects to the full recipe
pub fn recipe_card(RecipeProps { recipe }: &RecipeProps) -> Html {
    html! {
    <div class="recipe recipe-card">
        <h1 class="title">{recipe.recipe_name.clone()}</h1>
        <IngredientList ingredients={recipe.recipe_ingredients.clone()} />
        <Link<Route> classes={"button"} to={Route::Recipe { id: recipe.id.unwrap() }} >{format!("Detailed view of '{}'",recipe.recipe_name)}</ Link<Route>>


    </div>
    }
}
