use crate::{
    components::{
        input_component::{Input, InputType},
        new_step::NewSteps,
        recipe_component::StepList,
    },
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

use db::structs::{FullRecipe, Ingredient, Recipe, Step};
use log::{debug, error, info};
use yew::platform::spawn_local;
use yew_notifications::{use_notification, Notification};
use yew_router::hooks::use_navigator;

use super::RecipePartProps;

#[derive(Properties, Clone, PartialEq)]
pub struct EditStepProps {
    /// old recipe
    pub old_steps: Vec<Step>,
    /// handler for sending edited recipe to parent component
    pub edited_steps: Callback<Vec<Step>>,
    pub recipe_id: i32,
}

#[function_component(EditStep)]
pub fn edit_step(props: &EditStepProps) -> Html {
    let EditStepProps {
        recipe_id,
        edited_steps,
        old_steps,
    } = props;
    let add_step_state = use_state(|| false);
    let old_steps = old_steps.clone();
    let step_state = use_state(|| old_steps.clone());

    {
        //cloning current state
        let state = step_state.clone();
        let edited_steps = edited_steps.clone();
        // state will be used to emit Callback on change
        use_effect_with(state.clone(), move |steps| {
            let state = (*steps).clone();
            let steps = edited_steps.clone();
            // if step_state changes, send callback
            steps.emit((*state).clone())
        })
    }

    html! {<>
    <h1>{"Edit Steps"}</h1>
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
            <NewSteps {recipe_id}
            callback={
                // handling new item creation
                Callback::from(move |step:Step|{
                    let mut old_steps = old_steps.clone();
                    // appending newly created steps to old steps
                    old_steps.push(step);
                    // setting state (this will trigger use_effect)
                    step_state.set(old_steps)
            })}
            />
            </>
        }
    } else {html!{}}}

    </>
    }
}
