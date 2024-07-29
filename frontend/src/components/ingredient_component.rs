use db::structs::Ingredient;
use log::error;
use yew::{platform::spawn_local, prelude::*};
use yew_notifications::{use_notification, Notification};

use crate::{
    components::RecipeMode,
    functions::{recipe_functions::delete_ingredient, ApiResponse},
    DEFAULT_NOTIFICATION_DURATION,
};

use super::ItemProps;

#[function_component(IngredientItem)]
/// Component used to represent a Ingredient item
pub fn ingredient_component(props: &ItemProps<Ingredient>) -> Html {
    let ItemProps {
        item,
        mode,
        curr_focus,
        item_list: _,
    } = props;
    let edit_mode = mode.clone();
    let use_notification = use_notification::<Notification>();
    let focus_state = use_state(|| false);

    let handle_delete = {
        let curr_focus = curr_focus.clone();
        let ingredient = item.clone();
        let use_notification = use_notification.clone();
        Callback::from(move |_| {
            let ingredient = ingredient.clone();
            let use_notification = use_notification.clone();
            let curr_focus = curr_focus.clone();

            spawn_local(async move {
                let ingredient = ingredient.clone();

                match delete_ingredient(&ingredient).await {
                    Ok(ok_fetch) => match ok_fetch {
                        ApiResponse::ApiMessage(msg) => {
                            curr_focus.emit((RecipeMode::Delete, ingredient));

                            use_notification.spawn(Notification::new(
                                yew_notifications::NotificationType::Info,
                                "Sucess",
                                msg,
                                DEFAULT_NOTIFICATION_DURATION,
                            ));
                        }
                        ApiResponse::ApiError(err) => {
                            error!("API error: {:?}", err);
                            use_notification.spawn(Notification::new(
                                yew_notifications::NotificationType::Error,
                                "Error!",
                                err,
                                DEFAULT_NOTIFICATION_DURATION,
                            ));
                        }
                        _ => {}
                    },
                    Err(err) => {
                        error!("error: {:?}", err);
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
        })}}

        class="ingredient">
    {
    if edit_mode == RecipeMode::Edit && *focus_state{

    html!{
        <>
        <button onclick={{
            let ingredient = item.clone();
            let curr_focus = curr_focus.clone();
            Callback::from(move |_| {
                let ingredient = ingredient.clone();
                curr_focus.emit((RecipeMode::Edit,ingredient))
            })
        }}>{"Edit"}</button>
        <button
        onclick={handle_delete}
        >{"Delete"}</button>
        </>
    }}else {html!{<></>}}}

            {item.ingredient_name.clone()}
            {format!(" {} {}",
            item.ingredient_quantity,item.quantity_unit)}
        </div>
    }
}
