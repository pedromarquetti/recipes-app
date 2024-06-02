use db::structs::FullRecipe;
use log::{error, info};
use yew::{platform::spawn_local, prelude::*};

use crate::{
    components::recipe_component::RecipeComponent,
    functions::{recipe_functions::fetch_recipe, ApiResponse},
};

#[derive(Properties, PartialEq)]
pub struct RecipeProps {
    pub recipe_id: i32,
}

#[function_component(RecipePage)]
/// # Recipe page
///
/// Handles Displaying single recipe when user accesses {url}/recipe/{id}
pub fn recipe_page(props: &RecipeProps) -> Html {
    let recipe_id = props.recipe_id;

    // same as:
    // const [recipe,setRecipe] = useState(recipe)
    let recipe_state = use_state(|| FullRecipe::default());

    {
        let recipe_state = recipe_state.clone();
        use_effect_with((), move |_| {
            let recipe_state = recipe_state.clone();
            spawn_local(async move {
                match fetch_recipe(recipe_id).await {
                    Ok(ok_fetch) => match ok_fetch {
                        ApiResponse::OkRecipe(ok_recipe) => {
                            recipe_state.set(ok_recipe);
                            info!("recipe fetch ok!");
                        }
                        ApiResponse::ApiError(err) => {
                            error!("{:?}", err);
                        }
                        ApiResponse::ApiMessage(msg) => {
                            info!("{:?}", msg);
                        }
                    },
                    Err(err_fetching) => {
                        error!("{}", err_fetching)
                    }
                }
            });
        });
    }

    if recipe_state.recipe.id.is_some() {
        html! {
            <>
                <RecipeComponent full_recipe={(*recipe_state).clone()}/>
            </>
        }
    } else {
        html! {
        <>
        <h1>{"No recipe with this id!"}</h1>

        </>
        }
    }
}
