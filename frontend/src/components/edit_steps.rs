use crate::{
    components::{
        input_component::{Input, InputType},
        new_step::NewStepComponent,
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
    let step_state = use_state(|| Step::default());
    let use_notification = use_notification::<Notification>();
    let mode = use_state(|| RecipeMode::View);

    let step_name = use_node_ref();
    let step_instruction = use_node_ref();
    let step_duration_min = use_node_ref();

    {
        let state = step_state.clone();
        let mode = mode.clone();
        let old_part = old_part.clone();
        use_effect_with(old_part.clone(), move |i: &Step| {
            // setting ingredient_state
            state.set(i.clone());
            if old_part.id >= 0 {
                mode.set(RecipeMode::Edit)
            } else {
                mode.set(RecipeMode::View)
            }
        })
    }

    // handling form submit (Editing step)
    let handle_edit = {
        let state = step_state.clone();
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
                let state = state.clone();
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
                            ApiResponse::OkPart(_) => {
                                callback.emit((RecipeMode::Edit, step));
                                state.set(Default::default());
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

    html! {
    <div >

    // conditionally redering New Step
    {if *mode.clone() == RecipeMode::Edit || *mode.clone() == RecipeMode::View {

        html!{<button onclick={{
            let mode = mode.clone();Callback::from(move|_|{mode.set(RecipeMode::New)})
        }}>{"Add new Step"}</button>}

    } else {html!{}}
    }

    {
        match *mode {
        RecipeMode::Edit=>{
            html!{
            <>
                <h1>{"Edit Step"}</h1>
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
            </>
            }
        }
        RecipeMode::New=>{html!{
            <>
                <h1>{"New Step"}</h1>

            <NewStepComponent
            old_part={Step {
                recipe_id:recipe_id.clone(),
                ..Default::default()
            }}
            {callback}
            />

            <button onclick={{
                let mode = mode.clone(); Callback::from(move|_| {mode.set(RecipeMode::View)})
            }}>{"Cancel Add Steps"}</button>
            </>
        }}
        _=>{html!{}}
        }
    }
    </div>

    }
}
