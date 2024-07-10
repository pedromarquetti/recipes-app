use db::structs::Ingredient;
use log::{debug, error};
use web_sys::HtmlInputElement;

use yew::{platform::spawn_local, prelude::*};

use crate::{
    components::{
        input_component::{Input, InputType},
        RecipeMode, RecipePartProps,
    },
    functions::{recipe_functions::create_ingredient, ApiResponse},
    DEFAULT_NOTIFICATION_DURATION,
};
use yew_notifications::{use_notification, Notification};

#[function_component(NewIngredients)]
/// Ingredient list used for new recipes
///
/// Handles form inputs+changes
pub fn new_ingredient(props: &RecipePartProps<Ingredient>) -> Html {
    let RecipePartProps {
        callback,
        old_part,
        recipe_id: _,
    } = props;

    let use_notification = use_notification::<Notification>();

    let name_input = use_node_ref();
    let ingredient_quantity_input = use_node_ref();
    let quantity_unit_input = use_node_ref();

    // handling form submit (adding new ingredient to list)
    let onsubmit = {
        let old_part = old_part.clone();
        // cloning node ref
        let callback = callback.clone();

        let name_input = name_input.clone();
        let quantity_input = ingredient_quantity_input.clone();
        let unit_input = quantity_unit_input.clone();

        Callback::from(move |event: SubmitEvent| {
            let callback = callback.clone();

            event.prevent_default();

            let use_notification = use_notification.clone();
            // they have to be cloned because of the 'move' inside the closure

            // "cloned" represents the "ingredient_list_state" vec![]
            // it'll be used to push new values

            // getting form input values...
            let name = name_input.cast::<HtmlInputElement>().unwrap();
            let quantity = quantity_input.cast::<HtmlInputElement>().unwrap();
            let unit = unit_input.cast::<HtmlInputElement>().unwrap();

            let ingredient = Ingredient {
                id: None,
                recipe_id: old_part.recipe_id,
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
                                callback.emit((RecipeMode::New, ingredient));
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
    </div>
    }
}
