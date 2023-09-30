use db::structs::{FullRecipe, Recipe};
use log::{debug, error, info};
use yew::{platform::spawn_local, prelude::*};

use crate::{
    components::{ingredient_list_component::IngredientList, steps_component::StepsList},
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
    let recipe_state = use_state(|| FullRecipe {
        recipe: Recipe {
            id: None,
            user_id: None,
            recipe_name: "".into(),
            recipe_ingredients: vec![],
            recipe_observations: None,
        },
        steps: vec![],
    });

    {
        let recipe_state = recipe_state.clone();

        use_effect_with_deps(
            move |_| {
                // let recipe_state = recipe_state.clone();
                spawn_local(async move {
                    match fetch_recipe(recipe_id).await {
                        Ok(ok_fetch) => match ok_fetch {
                            ApiResponse::OkRecipe(ok_recipe) => {
                                debug!("{:?}", &ok_recipe.steps);
                                recipe_state.set(ok_recipe)
                            }
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
        <div class="recipe">
            <h1 class="title">{recipe_state.recipe.recipe_name.clone()}</h1>
            <IngredientList ingredients={recipe_state.recipe.recipe_ingredients.clone()}/>
            <StepsList steps={recipe_state.steps.clone()}/>


        </div>
    }
}
