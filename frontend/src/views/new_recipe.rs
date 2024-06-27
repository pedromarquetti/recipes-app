use db::structs::{FullRecipe, Ingredient, Step};
use log::{error, info};
use web_sys::HtmlInputElement;

use yew::{platform::spawn_local, prelude::*};
use yew_router::prelude::*;

use crate::{
    components::{
        input_component::{Input, InputType},
        new_ingredient::NewIngredients,
        new_step::NewSteps,
        recipe_component::RecipeComponent,
    },
    functions::{
        recipe_functions::{create_recipe, delete_recipe},
        ApiResponse,
    },
    views::Route,
    DEFAULT_NOTIFICATION_DURATION,
};

use yew_notifications::{use_notification, Notification};

#[function_component(NewRecipe)]
/// Handles recipe creation
pub fn new_recipe() -> Html {
    let use_notification = use_notification::<Notification>();
    let recipe_state = use_state(|| FullRecipe::default());
    let navigator = use_navigator().unwrap();

    let recipe_name_ref = use_node_ref();

    // <Ingredient> Callback handler
    let ingredient_callback: Callback<Vec<Ingredient>> = {
        // making a copy of the current recipe_state
        let recipe_state = recipe_state.clone();
        Callback::from(move |received_ingredients| {
            let recipe_state = recipe_state.clone();
            // local full_recipe (w/o UseStateHandle)
            let mut full_recipe = (*recipe_state).clone();

            // setting ingredients for the local recipe
            full_recipe.set_ingredients(received_ingredients);

            // updating local recipe_state with the local ingredients
            recipe_state.set(full_recipe);
        })
    };

    // <Step> Callback handler
    let step_callback: Callback<Vec<Step>> = {
        let recipe_state = recipe_state.clone();
        Callback::from(move |received_steps| {
            let recipe_state = recipe_state.clone();

            // local full_recipe (w/o UseStateHandle)
            let mut full_recipe = (*recipe_state).clone();

            // setting ingredients for the local recipe
            full_recipe.set_steps(received_steps);

            // updating local recipe_state with the local ingredients
            recipe_state.set(full_recipe);
        })
    };

    // on submit CallBack handler
    // this handles creating a new Recipe
    let onsubmit = {
        let use_notification = use_notification.clone();
        // cloning here so these variables can be used inside this block*
        let recipe_name = recipe_name_ref.clone();

        // let notifications_manager = use_notification::<Notification>();
        let recipe_state = recipe_state.clone();

        Callback::from(move |e: SubmitEvent| {
            let use_notification = use_notification.clone();
            let recipe_state = recipe_state.clone();

            // *necessary because of this 'move'
            e.prevent_default();

            // recipe name provided by user input
            let name = recipe_name
                .cast::<HtmlInputElement>()
                .expect("Invalid element!");

            // creating a FullRecipe
            let full_recipe = FullRecipe::default();
            let mut full_recipe = full_recipe.clone();

            // getting the Recipe from full_recipe
            let mut recipe = full_recipe.recipe.clone();

            recipe.set_name(name.value());

            // making request to API backend
            spawn_local(async move {
                match create_recipe(recipe).await {
                    Ok(api_response) => match api_response {
                        ApiResponse::OkRecipe(ok_recipe) => {
                            info!("recipe created! {:?}", ok_recipe);
                            full_recipe.set_recipe(ok_recipe.clone());
                            recipe_state.set(full_recipe);
                            use_notification.spawn(Notification::new(
                                yew_notifications::NotificationType::Info,
                                "Recipe created!",
                                format!("recipe { } created! ", ok_recipe.recipe_name),
                                DEFAULT_NOTIFICATION_DURATION,
                            ));
                        }
                        ApiResponse::ApiError(msg) => {
                            error!("error: {}", msg);
                            use_notification.spawn(Notification::new(
                                yew_notifications::NotificationType::Error,
                                "Error!",
                                msg,
                                DEFAULT_NOTIFICATION_DURATION,
                            ));
                        }
                        ApiResponse::ApiMessage(msg) => {
                            info!("{:?}", msg);
                            use_notification.spawn(Notification::new(
                                yew_notifications::NotificationType::Info,
                                "",
                                msg,
                                DEFAULT_NOTIFICATION_DURATION,
                            ));
                        }
                    },
                    Err(err) => {
                        error!("{:?}", err);
                        use_notification.spawn(Notification::new(
                            yew_notifications::NotificationType::Error,
                            "Error!",
                            err.to_string(),
                            DEFAULT_NOTIFICATION_DURATION,
                        ));
                    }
                }
            });
            name.set_value("")
        })
    };

    html! {
    <>
        <h1>{"New Recipe"}</h1>
        {
            if !recipe_state.recipe.id.is_none() {
                html! {
                    <>
                        <RecipeComponent full_recipe={(*recipe_state).clone()}/>
                    </>

                }
            } else {html!()}

        }

        {
        // only show new recipe form if recipe_state is_none()
            if recipe_state.recipe.id.is_none() {
            html! {
            <form {onsubmit} class="new-recipe">
                <Input
                    input_node_ref={recipe_name_ref.clone()}
                    input_placeholder="Recipe name"
                    input_name="recipe name"
                    is_required={true}
                    input_type={InputType::Text}/>
                <button >{"Create recipe"}</button>
            </form>
            }
            } else {html! {}}
        }

        {
            // only show <IngredientList> if there's a valid recipe at recipe_state
            if let Some(id) = recipe_state.recipe.id{
            // TODO! add Observation component

            html! {
                <>
                <NewIngredients callback={ingredient_callback} recipe_id={id}/>
                <NewSteps callback={step_callback} recipe_id={id}/>

    <h6>
            {format!("Note: when done, just click Home or go to")}
            <Link<Route> to={Route::Recipe {id:recipe_state.recipe.id.unwrap_or(1)}}>{
                format!("recipe {}",recipe_state.recipe.recipe_name)}
                </Link<Route>>

    <button
    onclick={
    Callback::from(
    move |_|{
        let recipe_state = recipe_state.clone();
        let use_notification = use_notification.clone();
        let navigator = navigator.clone();

    spawn_local(async move {
        match delete_recipe(recipe_state.recipe.clone()).await {
            Ok(ok_fetch)=>{
                match ok_fetch{
                    ApiResponse::ApiError(err)=>{
                        error!("API error: {:?}", err);
                        use_notification.spawn(Notification::new(
                            yew_notifications::NotificationType::Error,
                            "Error!",
                            err,
                            DEFAULT_NOTIFICATION_DURATION,
                            ));
                        },
                    ApiResponse::ApiMessage(msg) => {
                        info!("API message: {:?}", msg);
                        use_notification.spawn(Notification::new(
                        yew_notifications::NotificationType::Info,
                        "Sucess",
                        msg,
                        DEFAULT_NOTIFICATION_DURATION,

                    ));
                            navigator.push(&Route::Home);

            },
            _ => {} // this is a placeholder
        }}
                Err(err)=>{
                    use_notification.spawn(Notification::new(
                    yew_notifications::NotificationType::Error,
                    "Error!",
                    err.to_string(),
                    DEFAULT_NOTIFICATION_DURATION,
                ));
                }
            };
        });
    })}>{"cancel"}</button>

        </h6>
    </>
    }
    } else {html!()}
    }

    </>
    }
}
