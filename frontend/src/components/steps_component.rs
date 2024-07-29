use db::structs::Step;
use yew::{platform::spawn_local, prelude::*};
use yew_notifications::{use_notification, Notification};

use crate::{
    components::RecipeMode,
    functions::{recipe_functions::delete_step, ApiResponse},
    DEFAULT_NOTIFICATION_DURATION,
};

use super::ItemProps;

#[function_component(StepItem)]
/// Represents a base Step item
pub fn step_component(props: &ItemProps<Step>) -> Html {
    let ItemProps {
        item,
        mode,
        curr_focus,
        item_list: _,
    } = props;
    let edit_mode = mode.clone();
    let focus_state = use_state(|| false);
    let use_notification = use_notification::<Notification>();

    let handle_delete = {
        let item = item.clone();
        let curr_focus = curr_focus.clone();
        let use_notification = use_notification.clone();
        Callback::from(move |_| {
            let curr_focus = curr_focus.clone();
            let step = item.clone();
            let use_notification = use_notification.clone();

            spawn_local(async move {
                let step = step.clone();
                match delete_step(&step).await {
                    Ok(ok_fetch) => match ok_fetch {
                        ApiResponse::ApiError(err) => {
                            use_notification.spawn(Notification::new(
                                yew_notifications::NotificationType::Error,
                                "Error!",
                                err.to_string(),
                                DEFAULT_NOTIFICATION_DURATION,
                            ));
                        }
                        ApiResponse::ApiMessage(msg) => {
                            curr_focus.emit((RecipeMode::Delete, step));

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
                        use_notification.spawn(Notification::new(
                            yew_notifications::NotificationType::Error,
                            "Error!",
                            err.to_string(),
                            DEFAULT_NOTIFICATION_DURATION,
                        ));
                    }
                }
            })
        })
    };

    html! {
    <div
        onmouseleave={{
            let focus_state = focus_state.clone();
            Callback::from(move |_| {
            focus_state.set(false)
        })}}
        onmouseenter={{
            let focus_state = focus_state.clone();
            Callback::from(move |_| {
            focus_state.set(true)
        })}} class="step">
        {

        if edit_mode == RecipeMode::Edit && *focus_state {
        html!{
            <>
            <button onclick={{
                let step = item.clone();
                let curr_focus = curr_focus.clone();
                Callback::from(move |_| {
                    let step = step.clone();
                    curr_focus.emit((RecipeMode::Edit,step))
                })
            }}>{"Edit"}</button>
            <button onclick={handle_delete}>{"Delete"}</button>
        </>
    }} else {html!{<></>}}
        }
        <h4>{item.clone().step_name}</h4>
        <p>{format!("{}min - {}",item.clone().step_duration_min,item.clone().step_instruction)}</p>


    </div>
    }
}
