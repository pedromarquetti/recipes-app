use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TitleProps {
    pub title: String,
    pub owner: Option<i32>,
}

#[function_component(RecipeTitle)]
/// A recipe title consists of the recipe name + the creator
///
/// # TODO
///
/// 1. implement a user id to user name converter
pub fn recipe_title(TitleProps { title, owner }: &TitleProps) -> Html {
    html! {
            <div class="recipe-title">
            <h1>{title}</h1>
            <h6>{
                if let Some(user) = owner{
                    format!("by user {}",user)
                } else {
                    format!("by an anon. user")
                }
            }</h6>

        </div>
    }
}
