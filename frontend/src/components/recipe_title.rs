use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TitleProps {
    pub title: String,
    pub owner: String,
}

#[function_component(RecipeTitle)]
/// A recipe title consists of the recipe name + the creator
pub fn recipe_title(TitleProps { title, owner }: &TitleProps) -> Html {
    html! {
            <div class="recipe-title">
            <h1>{title}</h1>
            <h6>{
                if owner.is_empty() {
                    String::new()
                } else {
                format!("by user {}",owner)
            }

            }</h6>

        </div>
    }
}
