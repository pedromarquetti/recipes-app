use db::structs::{FullRecipe, Ingredient, RecipeTrait, Step};
use log::{error, info};
use web_sys::HtmlInputElement;

use yew::{platform::spawn_local, prelude::*};
use yew_router::prelude::*;

use crate::{
    components::{
        input_component::{Input, InputType},
        recipe_component::{IngredientList, RecipeComponent, StepList},
        RecipePartProps,
    },
    functions::{
        recipe_functions::{create_ingredient, create_recipe, create_step, delete_recipe},
        ApiResponse,
    },
    views::Route,
    DEFAULT_NOTIFICATION_DURATION,
};
use yew_notifications::{use_notification, Notification};

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
