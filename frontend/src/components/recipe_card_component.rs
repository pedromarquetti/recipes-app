use crate::{components::recipe_title::RecipeTitle, views::Route};
use db::structs::Recipe;
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
        <RecipeTitle recipe={recipe.clone()}/>

        <div class="card-interaction">
            <Link<Route> classes={"button"} to={Route::Recipe { id: recipe.id.unwrap() }} >{format!("Detailed view of '{}'",recipe.recipe_name)}</ Link<Route>>
        </div>


    </div>
    }
}
