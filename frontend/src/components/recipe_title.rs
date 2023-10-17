use db::structs::Recipe;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TitleProps {
    pub recipe: Recipe,
}

#[function_component(RecipeTitle)]
/// A recipe title consists of the recipe name + the creator
///
/// # TODO
///
/// 1. implement a user id to user name converter
pub fn recipe_title(TitleProps { recipe }: &TitleProps) -> Html {
    html! {
            <div class="recipe-title">
            <h1>{recipe.recipe_name.clone()}</h1>
            <h4>{
                if let Some(user) = recipe.user_id.clone(){
                    format!("by user {}",user)
                } else {
                    format!("by an anon. user")
                }
            }</h4>
        </div>
    }
}
