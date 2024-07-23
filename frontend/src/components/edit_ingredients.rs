use crate::{
    components::{
        input_component::{Input, InputType},
        new_ingredient::NewIngredients,
        RecipePartProps,
    },
    functions::{recipe_functions::update_ingredient, ApiResponse},
    DEFAULT_NOTIFICATION_DURATION,
};
use db::structs::Ingredient;
use log::error;
use web_sys::HtmlInputElement;
use yew::{platform::spawn_local, prelude::*};
use yew_notifications::{use_notification, Notification};

use super::RecipeMode;

#[function_component(EditIngredient)]
pub fn edit_ingredients(props: &RecipePartProps<Ingredient>) -> Html {
    let RecipePartProps {
        old_part,
        callback,
        recipe_id,
    } = props;
    let add_ingredient_state = use_state(|| false);
    let ingredient_state = use_state(|| Ingredient::default());
    let use_notification = use_notification::<Notification>();

    let name_input = use_node_ref();
    let ingredient_quantity_input = use_node_ref();
    let quantity_unit_input = use_node_ref();

    {
        let state = ingredient_state.clone();
        use_effect_with(old_part.clone(), move |i: &Ingredient| {
            // setting ingredient_state
            state.set(i.clone())
        })
    }

    // handling form submit (Editing ingredient)
    let handle_edit = {
        let old_part = old_part.clone();

        let callback = callback.clone();

        // cloning node ref
        let name_input = name_input.clone();
        let quantity_input = ingredient_quantity_input.clone();
        let unit_input = quantity_unit_input.clone();

        Callback::from(move |event: SubmitEvent| {
            let old_part = old_part.clone();

            let callback = callback.clone();
            event.prevent_default();

            let use_notification = use_notification.clone();
            // they have to be cloned because of the 'move' inside the closure

            // "cloned" represents the "ingredient_list_state" vec![]
            // it'll be used to push new values

            // getting form input values...
            let name_input = name_input.cast::<HtmlInputElement>().unwrap();
            let ingredient_name = {
                let val = name_input.value();
                if val.is_empty() {
                    old_part.ingredient_name
                } else {
                    val
                }
            };
            let quantity_input = quantity_input.cast::<HtmlInputElement>().unwrap();
            let ingredient_quantity = {
                let val = quantity_input.value();
                if val.is_empty() {
                    old_part.ingredient_quantity
                } else {
                    val.parse::<i32>().unwrap_or(old_part.ingredient_quantity)
                }
            };
            let unit_input = unit_input.cast::<HtmlInputElement>().unwrap();
            let quantity_unit = {
                let val = unit_input.value();
                if val.is_empty() {
                    old_part.quantity_unit
                } else {
                    val
                }
            };

            let ingredient = Ingredient {
                id: old_part.id,
                recipe_id: old_part.recipe_id,
                ingredient_name,
                ingredient_quantity,
                quantity_unit,
            };
            {
                let ingredient = ingredient.clone();
                spawn_local(async move {
                    let use_notification = use_notification.clone();

                    match update_ingredient(&ingredient).await {
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
                            ApiResponse::OkRecipe(_) => {
                                callback.emit((RecipeMode::Edit, ingredient));
                                use_notification.spawn(Notification::new(
                                    yew_notifications::NotificationType::Info,
                                    "Sucess",
                                    "Ingredient edited",
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
                name_input.set_value("");
                quantity_input.set_value("");
                unit_input.set_value("");
            }
        })
    };

    html! {
    <>
    <h1>{"Edit Ingredients"}</h1>
    {
    if (*ingredient_state).clone().id.is_some(){

        html!{
        <div class="new-ingredients">
        <h2>{format!("Editing ingredient {}",ingredient_state.ingredient_name)}</h2>

        <form onsubmit={handle_edit}>
            <Input
                input_node_ref={name_input.clone()}
                is_required={false}
                input_placeholder={format!("Current name is {}",ingredient_state.ingredient_name)}
                input_name="ingredient name"
                input_type={InputType::Text}
            />

            <Input
                input_node_ref={ingredient_quantity_input.clone()}
                input_placeholder={format!("Current quantity {}",ingredient_state.quantity_unit)}
                is_required={false}
                input_name="ingredient quantity"
                input_type={InputType::Number}
            />

            <Input
                input_node_ref={quantity_unit_input.clone()}
                input_placeholder={format!("Current value is  {}",ingredient_state.quantity_unit)}
                input_name="ingredient unit"
                is_required={false}
                input_type={InputType::Text}
            />
                <button>{format!("Update ingredient {}",ingredient_state.ingredient_name)}</button>

        </form>
    </div>
        }
    }else{html!{}}
    }

    // conditionally redering NewIngredients
    {if !*add_ingredient_state.clone() {
        html!{<button onclick={{let add_ingredient_state = add_ingredient_state.clone();Callback::from(move|_|{add_ingredient_state.set(true)})}}>{"Add new Ingredients"}</button>}

    } else {
        html!{}
    }}
    {if *add_ingredient_state.clone() {
        html!{<>
            <NewIngredients
            old_part={Ingredient{
                recipe_id:recipe_id.clone(),
                ..Default::default()
            }}
            {callback}

            />
            <button onclick={{let add_ingredient_state = add_ingredient_state.clone(); Callback::from(move|_| {add_ingredient_state.set(false)})}}>{"Cancel add new ingredients"}</button>
            </>

        }
    }
    else  {html!{}}
    }
    </>
    }
}
