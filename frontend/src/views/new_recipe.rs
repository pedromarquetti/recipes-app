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
        RecipeMode,
    },
    functions::{
        recipe_functions::{create_recipe, delete_recipe},
        ApiResponse,
    },
    views::Route,
    DEFAULT_NOTIFICATION_DURATION,
};

use yew_notifications::{use_notification, Notification};

#[derive(Properties, Clone, PartialEq)]
pub struct NewRecipeProps {
    /// old recipe
    pub full_recipe: FullRecipe,
    /// handler for sending edited recipe to parent component
    pub new_recipe_cb: Callback<FullRecipe>,
}

#[function_component(NewRecipe)]
/// Handles recipe creation
pub fn new_recipe(props: &NewRecipeProps) -> Html {
    let NewRecipeProps {
        full_recipe,
        new_recipe_cb,
    } = props;
    let use_notification = use_notification::<Notification>();
    let recipe_state = use_state(|| full_recipe.clone());
    let navigator = use_navigator().unwrap();
    {
        let state = recipe_state.clone();
        let new_recipe_cb = new_recipe_cb.clone();
        // detects changes in recipe_state and sends a cb.emit()
        use_effect_with(state.clone(), move |full_recipe_state| {
            let state = (*full_recipe_state).clone();
            let edited_recipe = new_recipe_cb.clone();
            edited_recipe.emit((*state).clone())
        });
    }

    let recipe_name_ref = use_node_ref();

    // `<Ingredient/>` Callback handler
    let ingredient_callback = {
        // making a copy of the current recipe_state
        let recipe_state = recipe_state.clone();
        Callback::from(move |(_, ingredient)| {
            let recipe_state = recipe_state.clone();
            // local full_recipe (w/o UseStateHandle)
            let full_recipe = (*recipe_state).clone();
            let mut ingredients = full_recipe.ingredients;
            ingredients.push(ingredient);
            // updating local recipe_state with the local ingredients
            recipe_state.set(FullRecipe {
                ingredients,
                ..(*recipe_state).clone()
            });
        })
    };

    // <Step> Callback handler
    let step_callback = {
        let recipe_state = recipe_state.clone();
        Callback::from(move |(_, step)| {
            let recipe_state = recipe_state.clone();

            // local full_recipe (w/o UseStateHandle)
            let full_recipe = (*recipe_state).clone();
            let mut steps = full_recipe.steps.clone();
            steps.push(step);

            // updating local recipe_state with the local steps
            recipe_state.set(FullRecipe {
                steps,
                ..(*recipe_state).clone()
            });
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
                match create_recipe(&recipe).await {
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
                <h1>{"New ingredient"}</h1>
                <NewIngredients
                callback={ingredient_callback}
                old_part={
                    {
                        Ingredient {
                            recipe_id:(*recipe_state).clone().recipe.id.expect("expected valid recipe_id at new recipe part"),
                            ..Default::default()
                        }
                    }
                }
                />
                <h1>{"New Step"}</h1>
                <NewSteps
                callback={step_callback}
                old_part={
                    Step{
                        recipe_id:(*recipe_state).clone().recipe.id.unwrap(),
                        ..Default::default()
                    }
                }/>

    <h6>
            {format!("Note: when done, just click Home or go to")}
            <Link<Route> to={Route::Recipe {id:recipe_state.recipe.id.unwrap_or(1)}}>{
                format!("recipe {}",recipe_state.recipe.recipe_name)}
                </Link<Route>>

    // TODO! add confirmation for deleting recipe
    <button
    onclick={
    Callback::from(
    move |_|{
        let recipe_state = recipe_state.clone();
        let use_notification = use_notification.clone();
        let navigator = navigator.clone();

    spawn_local(async move {
        match delete_recipe(&recipe_state.recipe.id).await {
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
