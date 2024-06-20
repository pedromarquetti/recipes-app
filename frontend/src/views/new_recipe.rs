use db::structs::{FullRecipe, Ingredient, Step};
use log::{error, info};
use web_sys::HtmlInputElement;

use yew::{platform::spawn_local, prelude::*};
use yew_router::prelude::*;

use crate::{
    components::{
        input_component::{Input, InputType},
        recipe_component::{IngredientList, RecipeComponent, StepList},
    },
    functions::{
        recipe_functions::{create_ingredient, create_recipe, create_step, delete_recipe},
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

#[derive(Properties, PartialEq)]
/// # TODO
///
/// put this elsewhere to be used globally...
pub struct RecipePartProps<T: std::cmp::PartialEq> {
    pub recipe_id: i32,
    pub callback: Callback<T>,
}
#[function_component(NewIngredients)]
/// Ingredient list used for new recipes
///
/// Handles form inputs+changes
pub fn new_ingredient(props: &RecipePartProps<Vec<Ingredient>>) -> Html {
    let use_notification = use_notification::<Notification>();
    let ingredient_list_state: UseStateHandle<Vec<Ingredient>> = use_state(|| vec![]);
    let recipe_id = props.recipe_id;

    let name_input = use_node_ref();
    let ingredient_quantity_input = use_node_ref();
    let quantity_unit_input = use_node_ref();

    // handling form submit (adding new ingredient to list)
    let onsubmit = {
        // cloning node ref
        let name_input = name_input.clone();
        let quantity_input = ingredient_quantity_input.clone();
        let unit_input = quantity_unit_input.clone();

        // cloning use_state
        let ingredient_list_state = ingredient_list_state.clone();

        let callback = props.callback.clone();

        Callback::from(move |event: SubmitEvent| {
            let use_notification = use_notification.clone();
            let callback = callback.clone();
            let ingredient_list_state = ingredient_list_state.clone();
            // they have to be cloned because of the 'move' inside the closure

            // "cloned" represents the "ingredient_list_state" vec![]
            // it'll be used to push new values
            let mut cloned_ingredient_list = (*ingredient_list_state).clone();

            // getting form input values...
            let name = name_input.cast::<HtmlInputElement>().unwrap();
            let quantity = quantity_input.cast::<HtmlInputElement>().unwrap();
            let unit = unit_input.cast::<HtmlInputElement>().unwrap();

            event.prevent_default();
            let ingredient = Ingredient {
                id: None,
                recipe_id,
                ingredient_name: name.value(),
                ingredient_quantity: quantity.value().parse::<i32>().unwrap_or(0),
                quantity_unit: unit.value(),
            };
            {
                let ingredient = ingredient.clone();
                spawn_local(async move {
                    let use_notification = use_notification.clone();

                    match create_ingredient(vec![ingredient.clone()]).await {
                        Ok(api_response) => match api_response {
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
                                // appending values to cloned vec![]
                                cloned_ingredient_list.push(ingredient);

                                // sending ingredient list to parent component
                                callback.emit(cloned_ingredient_list.clone());

                                // setting ingredient list state
                                ingredient_list_state.set(cloned_ingredient_list.clone());
                                use_notification.spawn(Notification::new(
                                    yew_notifications::NotificationType::Info,
                                    "Sucess",
                                    msg,
                                    DEFAULT_NOTIFICATION_DURATION,
                                ));
                            }
                            _ => {}
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
            }
        })
    };

    html! {
    <div class="new-ingredients">
        <h1>{"New ingredient"}</h1>
        <form {onsubmit}>
            <Input
                input_node_ref={name_input.clone()}
                is_required={true}

                input_placeholder="Ingredient name"
                input_name="ingredient name"
                input_type={InputType::Text}/>
            <Input
            input_node_ref={ingredient_quantity_input.clone()}
            input_placeholder="Ingredient quantity"
                is_required={true}

            input_name="ingredient quantity"
            input_type={InputType::Number}/>
            <Input
                input_node_ref={quantity_unit_input.clone()}
                input_placeholder="Ingredient unit (kg,g,L)"
                input_name="ingredient unit"
                is_required={true}

                input_type={InputType::Text}/>
                <button>{"New ingredient"}</button>

        </form>
        {
            if ingredient_list_state.len() ==0 {
                html! {
                <h6>{"ingredient list will appear here"}</h6>
                }
            }else {
                html! {
                <IngredientList ingredients={(*ingredient_list_state).clone()}/>
                }
            }
        }
    </div>
    }
}

#[function_component(NewSteps)]
pub fn new_recipe_step(props: &RecipePartProps<Vec<Step>>) -> Html {
    let use_notification = use_notification::<Notification>();
    let step_list_state: UseStateHandle<Vec<Step>> = use_state(|| vec![]);
    let callback = props.callback.clone();
    let recipe_id = props.recipe_id;

    let step_name = use_node_ref();
    let step_instruction = use_node_ref();
    let step_duration_min = use_node_ref();

    // handling form submit (adding new step to list)
    let onsubmit = {
        // cloning node ref
        let name_input = step_name.clone();
        let step_instruction = step_instruction.clone();
        let step_duration_min = step_duration_min.clone();

        // cloning use_state
        let step_list_state = step_list_state.clone();

        Callback::from(move |event: SubmitEvent| {
            let use_notification = use_notification.clone();
            let step_list_state = step_list_state.clone();

            // they have to be cloned because of the 'move' inside the closure

            // "cloned" represents the "step_list_state" vec![]
            // it'll be used to push new values
            let mut cloned_step_list = step_list_state.to_vec();

            // getting form input values...
            let name = name_input.cast::<HtmlInputElement>().unwrap();
            let step_instruction = step_instruction.cast::<HtmlInputElement>().unwrap();
            let step_duration_min = step_duration_min.cast::<HtmlInputElement>().unwrap();

            event.prevent_default();

            {
                let callback = callback.clone();
                let step = Step {
                    id: None,
                    recipe_id,
                    step_name: name.value(),
                    step_instruction: step_instruction.value(),
                    step_duration_min: step_duration_min.value().parse::<i32>().unwrap(),
                };

                spawn_local(async move {
                    let use_notification = use_notification.clone();

                    let step_list_state = step_list_state.clone();
                    let callback = callback.clone();
                    match create_step(vec![step]).await {
                        Ok(api_response) => match api_response {
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
                                // appending values to cloned vec![]
                                cloned_step_list.push(Step {
                                    id: None,
                                    recipe_id,
                                    step_name: name.value(),
                                    step_instruction: step_instruction.value(),
                                    step_duration_min: step_duration_min
                                        .value()
                                        .parse::<i32>()
                                        .unwrap(),
                                });
                                // setting cloned local vec as the current list_state
                                callback.emit(cloned_step_list.clone());
                                step_list_state.set(cloned_step_list);
                                info!("{:?}", msg);

                                use_notification.spawn(Notification::new(
                                    yew_notifications::NotificationType::Info,
                                    "Success!",
                                    msg,
                                    DEFAULT_NOTIFICATION_DURATION,
                                ));
                            }
                            _ => {}
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
            }
        })
    };

    html! {
    <div class="new-ingredients">
        <h1>{"New Step"}</h1>
        <form {onsubmit}>
            <Input
                input_node_ref={step_name.clone()}
                is_required={true}

                input_placeholder="step name"
                input_name="step name"
                input_type={InputType::Text}/>
            <Input
            input_node_ref={step_instruction.clone()}
            input_placeholder="Step instruction"
            is_required={true}
            input_name="Step instruction"
            input_type={InputType::Text}/>
            <Input
                input_node_ref={step_duration_min.clone()}
                input_placeholder="Step duration (mins)"
                input_name="duration"
                is_required={true}
                input_type={InputType::Number}/>
                <button>{"New ingredient"}</button>

        </form>
        {if step_list_state.len() == 0 {
            html! {
            <h6>{"List of steps to be added will appear here"}</h6>
            }
        }else {
            html! {
            <StepList step_list={(*step_list_state).clone()}/>
            }
        }}
    </div>
    }
}
