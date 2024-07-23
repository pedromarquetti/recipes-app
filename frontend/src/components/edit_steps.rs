use crate::{
    components::{
        input_component::{Input, InputType},
        new_step::NewSteps,
        RecipeMode,
    },
    functions::{recipe_functions::update_steps, ApiResponse},
    DEFAULT_NOTIFICATION_DURATION,
};
use web_sys::HtmlInputElement;
use yew::prelude::*;

use db::structs::Step;
use log::error;
use yew::platform::spawn_local;
use yew_notifications::{use_notification, Notification};

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

    // handling form submit (Editing step)
    let handle_edit = {
        let old_part = old_part.clone();
        let callback = callback.clone();

        // cloning node ref
        let name_input = step_name.clone();
        let step_instruction = step_instruction.clone();
        let step_duration_min = step_duration_min.clone();

        Callback::from(move |event: SubmitEvent| {
            let old_part = old_part.clone();
            let callback = callback.clone();
            event.prevent_default();

            let use_notification = use_notification.clone();
            // they have to be cloned because of the 'move' inside the closure

            // "cloned" represents the "ingredient_list_state" vec![]
            // it'll be used to push new values

            // getting form input values...
            // ignoring empty fields
            let name_input = name_input.cast::<HtmlInputElement>().unwrap();
            let step_name = {
                let value = name_input.value();
                if value.is_empty() {
                    old_part.step_name
                } else {
                    value
                }
            };
            let instruction_input = step_instruction.cast::<HtmlInputElement>().unwrap();
            let step_instruction = {
                let value = instruction_input.value();
                if value.is_empty() {
                    old_part.step_instruction
                } else {
                    value
                }
            };
            let duration_input = step_duration_min.cast::<HtmlInputElement>().unwrap();
            let step_duration_min = {
                let value = duration_input.value();
                if value.is_empty() {
                    old_part.step_duration_min
                } else {
                    value.parse::<i32>().unwrap_or(-1)
                }
            };
            let step = Step {
                id: old_part.id,
                recipe_id: old_part.recipe_id,
                step_name,
                step_instruction,
                step_duration_min,
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
                            ApiResponse::OkRecipe(_) => {
                                callback.emit((RecipeMode::Edit, step));
                                use_notification.spawn(Notification::new(
                                    yew_notifications::NotificationType::Info,
                                    "Sucess",
                                    "Step edited",
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
                instruction_input.set_value("");
                duration_input.set_value("");
            }
        })
    };

    html! {<>
    <h1>{"Edit Steps"}</h1>
    {
        if (*step_state).clone().id.is_some() && !*add_step_state{
        html!{
            <div class="new-ingredients">
        <form onsubmit={handle_edit}>
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
            <button>{format!("Update step {}",step_state.step_name)}</button>

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
