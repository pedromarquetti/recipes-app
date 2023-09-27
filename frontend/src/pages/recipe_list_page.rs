use db::structs::{FullRecipe, Recipe};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct RecipeListProps {
    pub recipes: Vec<FullRecipe>,
}

/// iterates through provided recipe list and displays them
#[function_component(RecipeList)]
pub fn recipe_list(RecipeListProps { recipes }: &RecipeListProps) -> Html {
    recipes
        .iter()
        .map(|recipe| {
            let ingredients:Html = recipe.recipe.recipe_ingredients.iter().map(|ingredient| html! {
                <li class="ingredient">
                    <p>{ingredient}</p>
                </li>
            }).collect();
            html! {
                <div class="recipe" key={recipe.recipe.id.unwrap()}>
                    <h1>{format!("Recipe: {}, by:{} (this is user id, user parsing will be implemented)",recipe.recipe.recipe_name,recipe.recipe.user_id.unwrap())}</h1>
                    <ul class="ingredient-list">
                    {ingredients}
                    </ul>


                </div>

            }
        })
        .collect()
}
