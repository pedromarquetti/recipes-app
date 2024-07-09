use db::structs::Ingredient;
use log::{error, info};
use yew::{platform::spawn_local, prelude::*};
use yew_notifications::{use_notification, Notification};

use crate::{
    functions::{recipe_functions::delete_ingredient, ApiResponse},
    DEFAULT_NOTIFICATION_DURATION,
};

#[derive(Properties, PartialEq)]
pub struct IngredientItemProp {
    pub ingredient: Ingredient,
    #[prop_or(false)]
    pub edit_mode: bool,
    #[prop_or_default]
    pub curr_focus: Callback<Ingredient>,
}

#[function_component(IngredientItem)]
/// Component used to represent a Ingredient item
pub fn ingredient_component(props: &IngredientItemProp) -> Html {
    let IngredientItemProp {
        ingredient,
        edit_mode,
        curr_focus,
    } = props;
    let edit_mode = edit_mode.clone();
    let use_notification = use_notification::<Notification>();
    let focus_state = use_state(|| false);

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
    if edit_mode && *focus_state{

    html!{
        <>
        <button onclick={{
            let ingredient = ingredient.clone();
            let curr_focus = curr_focus.clone();
            Callback::from(move |_| {
                let ingredient = ingredient.clone();
                curr_focus.emit(ingredient)
            })
        }}>{"Edit"}</button>
        <button
        onclick={{
            let ingredient = ingredient.clone();
            let use_notification = use_notification.clone();
            Callback::from(move |_|{
                let ingredient = ingredient.clone();
                let use_notification = use_notification.clone();
                spawn_local(async move {
                match delete_ingredient(&ingredient).await {
                    Ok(ok_fetch)=>{
                        match ok_fetch {
                            ApiResponse::ApiMessage(msg)=>{
                                info!("API message: {:?}", msg);
                                use_notification.spawn(Notification::new(
                                yew_notifications::NotificationType::Info,
                                "Sucess",
                                msg,
                                DEFAULT_NOTIFICATION_DURATION,
                            ));
                            }
                            ApiResponse::ApiError(err)=>{
                                error!("API error: {:?}", err);
                            use_notification.spawn(Notification::new(
                                yew_notifications::NotificationType::Error,
                                "Error!",
                                err,
                                DEFAULT_NOTIFICATION_DURATION,
                            ));
                            }
                            _=>{}
                        }
                    }
                    Err(err)=>{
                        error!("error: {:?}", err);
                    use_notification.spawn(Notification::new(
                        yew_notifications::NotificationType::Error,
                        "Error!",
                        err.to_string(),
                        DEFAULT_NOTIFICATION_DURATION,
                    ));
                }}
            })
        })
    }}
        >{"Delete"}</button>
        </>
    }}else {html!{<></>}}}

            {ingredient.ingredient_name.clone()}
            {format!(" {} {}",
            ingredient.ingredient_quantity,ingredient.quantity_unit)}
        </div>
    }
}
