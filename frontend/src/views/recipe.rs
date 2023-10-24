use db::structs::FullRecipe;
use log::error;
use yew::{platform::spawn_local, prelude::*};

use crate::{
    components::recipe_component::RecipeComponent,
    functions::recipe_functions::{fetch_recipe, ApiResponse},
};

#[derive(Properties, PartialEq)]
pub struct RecipeProps {
    pub recipe_id: i32,
}

#[function_component(RecipePage)]
/// Handles Displaying single recipe when user accesses {url}/recipe/{id}
pub fn recipe_page(props: &RecipeProps) -> Html {
    let recipe_id = props.recipe_id;

    // same as:
    // const [recipe,setRecipe] = useState(recipe)
    let recipe_state = use_state(|| FullRecipe::new());

    {
        let recipe_state = recipe_state.clone();

        use_effect_with_deps(
            move |_| {
                // let recipe_state = recipe_state.clone();
                spawn_local(async move {
                    match fetch_recipe(recipe_id).await {
                        Ok(ok_fetch) => match ok_fetch {
                            ApiResponse::OkRecipe(ok_recipe) => recipe_state.set(ok_recipe),
                            ApiResponse::ErrorMessage(err) => {
                                error!("{:?}", err)
                            }
                        },
                        Err(err_fetching) => {
                            error!("error fetching! {:#?}", err_fetching);
                        }
                    }
                });
                || ()
            },
            (),
        );
    }

    html! {
        <RecipeComponent recipe={(*recipe_state).clone()}/>
    }
}
