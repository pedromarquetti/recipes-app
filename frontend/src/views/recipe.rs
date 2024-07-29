use db::structs::FullRecipe;
use log::error;
use yew::{platform::spawn_local, prelude::*};
use yew_notifications::{use_notification, Notification};

use crate::{
    components::{recipe_component::RecipeComponent, RecipeMode},
    functions::{recipe_functions::fetch_recipe, ApiResponse},
    DEFAULT_NOTIFICATION_DURATION,
};

#[derive(Properties, PartialEq)]
pub struct RecipeProps {
    pub recipe_id: i32,
    #[prop_or(RecipeMode::View)]
    pub mode: RecipeMode,
}

#[function_component(RecipePage)]
/// # Recipe page
///
/// Handles Displaying single recipe when user accesses {url}/recipe/{id}
pub fn recipe_page(props: &RecipeProps) -> Html {
    let recipe_id = props.recipe_id;
    let use_notification = use_notification::<Notification>();

    // same as:
    // const [recipe,setRecipe] = useState(recipe)
    let recipe_state = use_state(|| FullRecipe::default());
    {
        let recipe_state = recipe_state.clone();
        use_effect_with(props.mode.clone(), move |mode| {
            if let RecipeMode::View = mode {
                let recipe_state = recipe_state.clone();
                spawn_local(async move {
                    let use_notification = use_notification.clone();

                    match fetch_recipe(&recipe_id).await {
                        Ok(ok_fetch) => match ok_fetch {
                            ApiResponse::OkPart(ok_recipe) => {
                                recipe_state.set(ok_recipe);
                            }
                            ApiResponse::ApiError(err) => {
                                error!("{:?}", err);
                                use_notification.spawn(Notification::new(
                                    yew_notifications::NotificationType::Error,
                                    "Error!",
                                    err,
                                    DEFAULT_NOTIFICATION_DURATION,
                                ));
                            }
                            _ => {}
                        },
                        Err(err) => {
                            error!("{}", err);
                            use_notification.spawn(Notification::new(
                                yew_notifications::NotificationType::Error,
                                "Error!",
                                err.to_string(),
                                DEFAULT_NOTIFICATION_DURATION,
                            ));
                        }
                    }
                });
            }
        });
    }

    match props.mode {
        RecipeMode::View => {
            if recipe_state.recipe.id > -1 {
                html! {
                    <>
                        <RecipeComponent mode={RecipeMode::View} full_recipe={(*recipe_state).clone()}/>
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
        RecipeMode::New => {
            html! {<RecipeComponent full_recipe={FullRecipe::default()} mode={RecipeMode::New}/>}
        }
        _ => {
            // omitting Edit mode
            html! {}
        }
    }
}
