use db::structs::Step;
use log::{error, info};
use web_sys::HtmlInputElement;

use yew::{platform::spawn_local, prelude::*};

use crate::{
    components::{
        input_component::{Input, InputType},
        recipe_component::StepList,
        RecipePartProps,
    },
    functions::{recipe_functions::create_step, ApiResponse},
    DEFAULT_NOTIFICATION_DURATION,
};

use yew_notifications::{use_notification, Notification};

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
