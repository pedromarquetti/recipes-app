use crate::{
    components::input_component::{Input, InputType},
    functions::{
        recipe_functions::{delete_recipe, update_recipe},
        ApiResponse,
    },
    views::Route,
    DEFAULT_NOTIFICATION_DURATION,
};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

use db::structs::{FullRecipe, Ingredient, Recipe};
use log::{debug, error, info};
use yew::platform::spawn_local;
use yew_notifications::{use_notification, Notification};
use yew_router::hooks::use_navigator;

#[derive(Properties, Clone, PartialEq)]
pub struct EditRecipeProps {
    /// old recipe
    pub full_recipe: FullRecipe,
    /// handler for sending edited recipe to parent component
    pub edited_recipe: Callback<FullRecipe>,
    pub close: Callback<()>,
}

#[function_component(EditRecipe)]
pub fn edit_recipe(props: &EditRecipeProps) -> Html {
    let EditRecipeProps {
        full_recipe,
        close,
        edited_recipe,
    } = props;
    let old_recipe = full_recipe.clone();
    let recipe_state = use_state(|| old_recipe.clone());

    {
        let state = recipe_state.clone();
        let edited_recipe = edited_recipe.clone();
        // detects changes in recipe_state and sends a cb.emit()
        use_effect_with(state.clone(), move |full_recipe_state| {
            let state = (*full_recipe_state).clone();
            let edited_recipe = edited_recipe.clone();

            edited_recipe.emit((*state).clone())
        });
    }

    let recipe = old_recipe.recipe.clone();
    let new_name_ref = use_node_ref();

    let navigator = use_navigator().unwrap();
    let use_notification = use_notification::<Notification>();

    let close = close.clone();

    let rename = {
        let recipe_state = recipe_state.clone();
        let use_notification = use_notification.clone();
        let new_name = new_name_ref.clone();

        Callback::from(move |e: SubmitEvent| {
            let recipe_state = recipe_state.clone();

            let use_notification = use_notification.clone();
            e.prevent_default();
            let new_name = new_name.cast::<HtmlInputElement>().unwrap();
            // cloning current state
            let mut new_fullrecipe = (*recipe_state).clone();
            let old_name = new_fullrecipe.clone().recipe.recipe_name;

            // creating new recipe
            let mut new_local_recipe = new_fullrecipe.clone().recipe;

            // setting name for local recipe
            new_local_recipe.set_name(new_name.value());

            let new_value = new_name.clone();
            spawn_local(async move {
                let recipe_state = recipe_state.clone();
                match update_recipe(&new_local_recipe).await {
                    // updating recipe
                    Ok(ok_fetch) => {
                        // handling API Response
                        match ok_fetch {
                            ApiResponse::ApiMessage(msg) => {
                                info!("API message: {:?}", msg);
                                use_notification.spawn(Notification::new(
                                    yew_notifications::NotificationType::Info,
                                    "Sucess",
                                    msg,
                                    DEFAULT_NOTIFICATION_DURATION,
                                ));
                                // setting recipe for local full_recipe
                                new_fullrecipe.set_recipe(new_local_recipe);
                                recipe_state.set(new_fullrecipe);
                            }
                            ApiResponse::ApiError(err) => {
                                error!("API error: {:?}", err);
                                new_local_recipe.set_name(old_name);
                                use_notification.spawn(Notification::new(
                                    yew_notifications::NotificationType::Error,
                                    "Error!",
                                    err,
                                    DEFAULT_NOTIFICATION_DURATION,
                                ));
                            }
                            _ => {}
                        }
                    }

                    // API fetch error!
                    Err(err) => {
                        use_notification.spawn(Notification::new(
                            yew_notifications::NotificationType::Error,
                            "Error!",
                            err.to_string(),
                            DEFAULT_NOTIFICATION_DURATION,
                        ));
                    }
                }
            });
            new_value.set_value("")
        })
    };

    html! {
    <div class="container recipe">
        <h1>{format!("Editing recipe {}",recipe.recipe_name)}</h1>
        <div class="edit-container">
        <form onsubmit={rename}>
            <Input
                    input_node_ref={new_name_ref}
                    is_required={true}
                    input_placeholder="Rename recipe"
                    input_name="recipe-rename"
                    input_type={InputType::Text}
                    />
            <button >{"Rename"}</button>
        </form>

        </div>
        <div class="edit-actions">
            // delete recipe
            <button
            onclick={
            Callback::from(move |_|{
                let navigator = navigator.clone();
                let recipe = recipe.clone();
                let use_notification = use_notification.clone();
                spawn_local(async move {
                    let recipe = recipe.clone();
                    match delete_recipe(&recipe.id).await{
                    Ok(api_res)=>{
                        match api_res {
                            ApiResponse::ApiMessage(msg)=>{
                                info!("API message: {:?}", msg);
                                use_notification.spawn(Notification::new(
                                    yew_notifications::NotificationType::Info,
                                    "Sucess",
                                    msg,
                                    DEFAULT_NOTIFICATION_DURATION,
                                ));
                                navigator.push(&Route::Home)
                            },
                            ApiResponse::ApiError(err) => {
                                error!("API error: {:?}", err);
                                use_notification.spawn(Notification::new(
                                    yew_notifications::NotificationType::Error,
                                    "Error!",
                                    err,
                                    DEFAULT_NOTIFICATION_DURATION,
                                ));
                            },
                        _ => {} // this is a placeholder
                        }},
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
            }
            >{"Delete recipe"}</button>

            <button onclick={move |_|{
            let close = close.clone();
            close.emit(())
        }}>{"cancel"}</button>
        </div>
    </div>

    }
}
