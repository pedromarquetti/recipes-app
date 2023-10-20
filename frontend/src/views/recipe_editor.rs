use db::structs::FullRecipe;
use yew::prelude::*;

#[derive(PartialEq)]
/// Recipe modifier can use these modes
pub enum ViewMode {
    Add,
    Edit,
}
#[derive(PartialEq, Properties)]
pub struct RecipeEditorProps {
    pub view_mode: ViewMode,
    pub recipe: FullRecipe,
}

#[function_component(RecipeModifier)]
/// responsible for creating / editing / updating recipes
pub fn recipe_modifier(props: &RecipeEditorProps) -> Html {
    let RecipeEditorProps { view_mode, recipe } = props;
    match view_mode {
        ViewMode::Add => {
            html! {
                <>{format!("todo! implement add recipe")}</>
            }
        }
        ViewMode::Edit => {
            html! {
                <>{format!("todo! implement recipe editor")}</>
            }
        }
    }
}
