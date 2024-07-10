use std::default;

use crate::{
    components::{
        input_component::{Input, InputType},
        new_step::NewSteps,
        recipe_component::StepList,
        RecipeMode,
    },
    functions::{
        recipe_functions::{delete_recipe, update_recipe, update_steps},
        ApiResponse,
    },
    views::Route,
    DEFAULT_NOTIFICATION_DURATION,
};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

use db::structs::{FullRecipe, Ingredient, Recipe, Step};
use log::{debug, error, info};
use yew::platform::spawn_local;
use yew_notifications::{use_notification, Notification};
use yew_router::hooks::use_navigator;

use super::RecipePartProps;

#[function_component(EditStep)]
pub fn edit_step(props: &RecipePartProps<Step>) -> Html {
    let RecipePartProps {
        callback,
        old_part,
        recipe_id,
    } = props;
    let add_step_state = use_state(|| false);
    let step_state = use_state(|| old_part.clone());
    let use_notification = use_notification::<Notification>();

    let step_name = use_node_ref();
    let step_instruction = use_node_ref();
    let step_duration_min = use_node_ref();

    {
        let state = step_state.clone();
        use_effect_with(old_part.clone(), move |i: &Step| {
            // setting ingredient_state
            state.set(i.clone())
        })
    }

    // handling form submit (editing ingredient to list)
    let onsubmit = {
        let old_part = old_part.clone();
        let callback = callback.clone();

        // cloning node ref
        let name_input = step_name.clone();
        let step_instruction = step_instruction.clone();
        let step_duration_min = step_duration_min.clone();

        Callback::from(move |event: SubmitEvent| {
            let callback = callback.clone();
            event.prevent_default();

            let use_notification = use_notification.clone();
            // they have to be cloned because of the 'move' inside the closure

            // "cloned" represents the "ingredient_list_state" vec![]
            // it'll be used to push new values

            // getting form input values...
            let name = name_input.cast::<HtmlInputElement>().unwrap();
            let step_instruction = step_instruction.cast::<HtmlInputElement>().unwrap();
            let step_duration_min = step_duration_min.cast::<HtmlInputElement>().unwrap();

            let step = Step {
                id: None,
                recipe_id: old_part.recipe_id,
                step_name: name.value(),
                step_instruction: step_instruction.value(),
                step_duration_min: step_duration_min
                    .value()
                    .parse::<i32>()
                    .map_err(|err| {
                        let use_notification = use_notification.clone();
                        use_notification.spawn(Notification::new(
                            yew_notifications::NotificationType::Error,
                            "Error!",
                            err.to_string(),
                            DEFAULT_NOTIFICATION_DURATION,
                        ));
                    })
                    .unwrap_or(-1),
            };
            {
                let step = step.clone();
                spawn_local(async move {
                    let use_notification = use_notification.clone();

                    match update_steps(&step).await {
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
                                callback.emit((RecipeMode::Edit, step));
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

    html! {<>
    <h1>{"Edit Steps"}</h1>
    {
        if (*step_state).clone().id.is_some(){
        html!{
            <div class="new-ingredients">
        <form {onsubmit}>
            <Input
                input_node_ref={step_name.clone()}
                is_required={false}
                input_placeholder={(*step_state).clone().step_name}
                input_name="step name"
                input_type={InputType::Text}
            />
            <Input
                input_node_ref={step_instruction.clone()}
                input_placeholder={(*step_state).clone().step_instruction}
                is_required={false}
                input_name="Step instruction"
                input_type={InputType::Text}
            />
            <Input
                input_node_ref={step_duration_min.clone()}
                input_placeholder={format!("{}",(*step_state).clone().step_duration_min)}
                input_name="duration"
                is_required={false}
                input_type={InputType::Number}
            />
            <button>{"New step"}</button>

        </form>
    </div>
        }
    }else{html!{}}
    }



    {if !*add_step_state.clone() {
        html!{
            <button onclick={{let add_step_state=add_step_state.clone();Callback::from(move|_|{add_step_state.set(true)})}}>{"Add Steps"}</button>
        }
    } else {
        html!{}
    }}
    {if *add_step_state.clone() {
        html!{<>
            <button onclick={{let add_step_state=add_step_state.clone();Callback::from(move|_|{add_step_state.set(false)})}}>{"Cancel Add Steps"}</button>
            <NewSteps
            old_part={Step {
                recipe_id:recipe_id.clone(),
                ..Default::default()
            }}
            {callback}
            />
            </>
        }
    } else {html!{}}}

    </>
    }
}
