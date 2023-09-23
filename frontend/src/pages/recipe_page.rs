use db::structs::{FullRecipe, Recipe};
use log::{error, info};
use yew::{platform::spawn_local, prelude::*};

use crate::functions::recipe_functions::{fetch_recipe, ApiResponse};

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
    // let recipe = recipe_state.clone();

    use_effect_with_deps(
        move |_| {
            spawn_local(async move {
                match fetch_recipe(recipe_id).await {
                    Ok(ok_fetch) => {
                        info!("ok fetch,");
                        info!("TODO: implement recipe handling here")
                        // match ok_fetch {
                        // ApiResponse::ErrorMessage(msg) => error!("{:?}", msg),
                        // ApiResponse::OkRecipe(ok) => {
                        //     let recipe: FullRecipe = ok.into();
                        //     info!("{:?}", ok)
                        // }
                    }
                    Err(err_fetching) => {
                        error!("error fetching! {:#?}", err_fetching);
                    }
                }
            });
            || ()
        },
        (),
    );

    html! {<></>
    }
}
