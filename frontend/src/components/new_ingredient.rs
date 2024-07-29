use db::structs::{Ingredient, NewIngredient};
use log::error;
use web_sys::{wasm_bindgen::UnwrapThrowExt, FormData, HtmlFormElement, HtmlInputElement};

use yew::{platform::spawn_local, prelude::*};

use crate::{
    components::{
        input_component::{Input, InputType},
        units::MeasuringUnits,
        RecipeMode, RecipePartProps,
    },
    functions::{recipe_functions::create_ingredient, ApiResponse},
    DEFAULT_NOTIFICATION_DURATION,
};
use yew_notifications::{use_notification, Notification};

#[function_component(NewIngredientComponent)]
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

    // handling form submit (adding new ingredient to list)
    let handle_new_ingredient = {
        let old_part = old_part.clone();
        let callback = callback.clone();

        // cloning node ref
        let name_input = name_input.clone();
        let quantity_input = ingredient_quantity_input.clone();

        Callback::from(move |e: SubmitEvent| {
            let callback = callback.clone();

            e.prevent_default();
            let unit = FormData::new_with_form(&e.target_dyn_into::<HtmlFormElement>().unwrap())
                .unwrap_throw()
                .get("measuring_units")
                .as_string()
                .unwrap_throw();

            let use_notification = use_notification.clone();
            // they have to be cloned because of the 'move' inside the closure

            // "cloned" represents the "ingredient_list_state" vec![]
            // it'll be used to push new values

            // getting form input values...
            let name = name_input.cast::<HtmlInputElement>().unwrap();
            let quantity = quantity_input.cast::<HtmlInputElement>().unwrap();

            let ingredient = NewIngredient {
                recipe_id: old_part.recipe_id,
                ingredient_name: name.value(),
                ingredient_quantity: quantity.value().parse::<i32>().unwrap_or(0),
                quantity_unit: unit,
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
                            ApiResponse::OkPart(i) => {
                                callback.emit((RecipeMode::New, i[0].clone()));
                                use_notification.spawn(Notification::new(
                                    yew_notifications::NotificationType::Info,
                                    "Sucess",
                                    "Ingredient added!",
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
    <div >
        <form id={"new-ingredient"} onsubmit={handle_new_ingredient}>
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

            <MeasuringUnits
            id={"new-ingredient"}
            />
            <button>{"New ingredient"}</button>

        </form>
    </div>
    }
}
