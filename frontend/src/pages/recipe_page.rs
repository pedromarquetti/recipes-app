use db::structs::{FullRecipe, Recipe};

use gloo_net::{http::Request, Error as GlooError};
use log::{error, info};
use serde_json::{json, Value};
use yew::{platform::spawn_local, prelude::*};

#[derive(Properties, PartialEq)]
pub struct RecipeProps {
    pub recipe_id: i32,
}

async fn fetch_recipe(recipe_id: i32) -> Result<Value, GlooError> {
    let req = Request::post("http://localhost:3000/api/view/recipe/")
        .json(&json!({
            "id":recipe_id,
            "recipe_name": "",
            "recipe_ingredients": [""],
        }))?
        .send()
        .await?;
    // TODO make it less generic
    let res = req.json::<Value>().await?;

    Ok(res)
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
                        info!("ok fetch: {}", ok_fetch);
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
