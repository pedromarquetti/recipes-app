use db::structs::{FullRecipe, Recipe};
use log::{debug, error, info};
use yew::{platform::spawn_local, prelude::*};

use crate::{
    components::recipe_card_component::RecipeCard,
    functions::recipe_functions::fuzzy_list_recipe,
    views::error::{ErrorPage, ErrorType},
};

#[derive(Properties, PartialEq)]
pub struct RecipeListProps {
    pub recipe_name: String,
}

/// iterates through provided recipe list and displays them
#[function_component(RecipeList)]
pub fn recipe_list(RecipeListProps { recipe_name }: &RecipeListProps) -> Html {
    let name = recipe_name.clone();
    let recipe_state = use_state(|| vec![]);
    {
        let recipe_state = recipe_state.clone();
        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    match fuzzy_list_recipe(name).await {
                        Ok(ok_recipes) => recipe_state.set(ok_recipes),
                        Err(err) => {
                            error!("err");
                        }
                    }
                });
                || ()
            },
            (),
        )
    }
    if recipe_state.clone().len() == 0 {
        return html! {
        <ErrorPage text={"No Recipes Found"} error_type={ErrorType::NotFound}/>
        };
    }
    let list: Html = recipe_state
        .iter()
        .map(|recipe| {
            html! {

                <li>
                    <RecipeCard recipe={recipe.clone()}/>
                </li>
            }
        })
        .collect();

    html! {
        <>
            <h1>{format!("Found {} recipes",recipe_state.clone().len())}</h1>
            <div class="recipes-list">
            <ul>
            {list}
            </ul>
            </div>
        </>
    }
}
