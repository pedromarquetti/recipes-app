use db::structs::{NewStep, Step};
use log::error;
use web_sys::HtmlInputElement;

use yew::{platform::spawn_local, prelude::*};

use crate::{
    components::{
        input_component::{Input, InputType},
        RecipeMode, RecipePartProps,
    },
    functions::{recipe_functions::create_step, ApiResponse},
    DEFAULT_NOTIFICATION_DURATION,
};

use yew_notifications::{use_notification, Notification};

#[function_component(NewStepComponent)]
pub fn new_recipe_step(props: &RecipePartProps<Step>) -> Html {
    let RecipePartProps {
        old_part,
        recipe_id: _,
        callback,
    } = props;
    let use_notification = use_notification::<Notification>();
    let callback = callback.clone();

    let step_name = use_node_ref();
    let step_instruction = use_node_ref();
    let step_duration_min = use_node_ref();

    // handling form submit (adding new step to list)
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

            // getting form input values...
            let name = name_input.cast::<HtmlInputElement>().unwrap();
            let step_instruction = step_instruction.cast::<HtmlInputElement>().unwrap();
            let step_duration_min = step_duration_min.cast::<HtmlInputElement>().unwrap();

            let step = NewStep {
                recipe_id: old_part.recipe_id,
                step_name: name.value(),
                step_instruction: step_instruction.value(),
                step_duration_min: step_duration_min.value().parse::<i32>().unwrap(),
            };
            {
                let callback = callback.clone();
                spawn_local(async move {
                    let use_notification = use_notification.clone();

                    let callback = callback.clone();

                    match create_step(vec![&step]).await {
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
                            ApiResponse::OkPart(step) => {
                                callback.emit((RecipeMode::New, step[0].clone()));
                                use_notification.spawn(Notification::new(
                                    yew_notifications::NotificationType::Info,
                                    "Success!",
                                    "Ingredient created",
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
                <button>{"New step"}</button>

        </form>

    </div>
    }
}
