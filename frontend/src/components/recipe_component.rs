use db::structs::{FullRecipe, Ingredient, Step};
use yew::{platform::spawn_local, prelude::*};
use yew_notifications::{use_notification, Notification};
use yew_router::hooks::use_navigator;

use crate::{
    components::{
        edit_mode::EditRecipe, ingredient_component::IngredientItem, recipe_title::RecipeTitle,
        steps_component::StepItem,
    },
    functions::{recipe_functions::check_edit_permission, ApiResponse},
    DEFAULT_NOTIFICATION_DURATION,
};

#[derive(Properties, PartialEq)]
pub struct RecipeProps {
    pub full_recipe: FullRecipe,
}
#[function_component(RecipeComponent)]
/// Base Recipe component
pub fn recipe_component(RecipeProps { full_recipe }: &RecipeProps) -> Html {
    let recipe_state = use_state(|| full_recipe.clone());
    let use_notification = use_notification::<Notification>();
    let recipe = recipe_state.recipe.clone();
    let ingredients = recipe_state.ingredients.clone();
    let steps = recipe_state.steps.clone();

    let edit_mode = use_state(|| false);

    let onclick = {
        let edit_mode = edit_mode.clone();
        Callback::from(move |_| {
            let edit_mode = edit_mode.clone();
            let use_notification = use_notification.clone();
            spawn_local(async move {
                match check_edit_permission(&recipe.id.unwrap_or(-1)).await {
                    Ok(ok_fetch) => match ok_fetch {
                        ApiResponse::ApiError(err) => {
                            use_notification.spawn(Notification::new(
                                yew_notifications::NotificationType::Error,
                                "Error!",
                                err,
                                DEFAULT_NOTIFICATION_DURATION,
                            ));
                        }
                        ApiResponse::ApiMessage(_) => {
                            edit_mode.set(true);
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
            });
        })
    };

    html! {<>
        <div class="recipe">
            <button {onclick}>{"Edit recipe"}</button>
            <RecipeTitle owner={full_recipe.recipe_owner_name.clone()} title={recipe.recipe_name}/>
            <IngredientList ingredients={
                ingredients
            }/>
            <StepList step_list={steps}/>
        </div>

    {
    if *edit_mode {
        html! {
            <EditRecipe
            full_recipe={full_recipe.clone()}
            edited_recipe={
                Callback::from(move |edited_recipe|{
                    let recipe_state = recipe_state.clone();
                    recipe_state.set(edited_recipe);
                })
            }
            close={Callback::from(move|_| {
                edit_mode.set(false);
            })}/>
        }
    } else {
        html! {}
    }
    }
    </>
    }
}

#[derive(Properties, PartialEq)]
pub struct StepListProps {
    pub step_list: Vec<Step>,
}
#[function_component(StepList)]
pub fn step_list(steps: &StepListProps) -> Html {
    let l: Vec<Html> = steps
        .step_list
        .iter()
        .map(|step: &Step| {
            // note: calling unwrap on step.id because step will always receive an id
            html! {
                <li id={format!("step-{}",step.id.unwrap_or(steps.step_list.len().try_into().expect("invalid len()")))} class="step">
                    <StepItem step={step.clone()}/>
                </li>
            }
        })
        .collect();

    html! {
    <>
        <h2 >{"Steps"}</h2>
        <ol class="list">
            {l}
        </ol>
    </>
    }
}

#[derive(Properties, PartialEq)]
pub struct IngredientsListProps {
    pub ingredients: Vec<Ingredient>,
}
#[function_component(IngredientList)]
/// Represents a list of ingredients
///
/// Uses a vec![Ingredient] as a prop.
pub fn ingredients_list(IngredientsListProps { ingredients }: &IngredientsListProps) -> Html {
    let ingredient_list: Vec<Html> = ingredients
        .iter()
        .map(|ingredient| {
            html! {
                <li id={format!{"ingredient-{}",ingredient.id.unwrap_or(ingredients.len().try_into().expect("invelid len()"))}}>
                    <IngredientItem ingredient={ingredient.clone()} />
                </li>
            }
        })
        .collect();
    html! {
        <>
            <h2 class="ingredients">{"Ingredients"}</h2>
            <ul class="list">
                {ingredient_list}
            </ul>
        </>
    }
}
