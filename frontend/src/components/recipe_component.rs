use db::structs::{FullRecipe, Ingredient, Step};
use yew::{platform::spawn_local, prelude::*};
use yew_notifications::{use_notification, Notification};

use crate::{views::new_recipe::NewRecipe,
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
    #[prop_or(RecipeComponentMode::View)]
    pub mode:RecipeComponentMode
}
#[derive(PartialEq,Clone,Debug)]
pub enum RecipeComponentMode {
    View,
    Edit,
    New
}

#[function_component(RecipeComponent)]
/// Base Recipe component
pub fn recipe_component(props: &RecipeProps) -> Html {
    let RecipeProps { full_recipe, mode }=props;
    let recipe_state = use_state(|| full_recipe.clone());
    let use_notification = use_notification::<Notification>();    
    let ingredient_to_edit = use_state(||Ingredient::default());
    let step_to_edit = use_state(||Step::default());
    let mode_state = use_state(||mode.clone());
    let edit_mode = use_state(|| false);
    
    let full_recipe = (*recipe_state).clone();
    let recipe = full_recipe.recipe.clone();
    let ingredients = full_recipe.ingredients.clone();
    let steps = full_recipe.steps.clone();

    let onclick = {
        let edit_mode = edit_mode.clone();
        let mode_state = mode_state.clone();
        Callback::from(move |_| {
            let edit_mode = edit_mode.clone();
            let mode_state =mode_state.clone();
            let use_notification = use_notification.clone();
            spawn_local(async move {
                let mode_state = mode_state.clone();
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
                            mode_state.set(RecipeComponentMode::Edit);

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
        {
            if RecipeComponentMode::View  == (*mode_state).clone()|| RecipeComponentMode::Edit  == (*mode_state).clone(){
                html!{<button {onclick}>{"Edit Recipe"}</button>}
            } else{html!{}}
        }
            <RecipeTitle owner={full_recipe.recipe_owner_name.clone()} title={recipe.recipe_name}/>
            <IngredientList 
            curr_focus={
                Callback::from(move |i:Ingredient|{
                    ingredient_to_edit.set(i)
                })
            } 
            edit_mode={(*edit_mode).clone()} 
            ingredients={ingredients}/>
            
            <StepList 
            curr_focus={Callback::from(move |s:Step|{
                step_to_edit.set(s)
            })}
            edit_mode={(*edit_mode).clone()}
            step_list={steps}
            />
        </div>

        {
        match *mode_state {
            RecipeComponentMode::Edit=>{
                html!{
                <EditRecipe
                full_recipe={full_recipe.clone()}
                edited_recipe={
                    Callback::from(move |edited_recipe:FullRecipe|{
                        let recipe_state = recipe_state.clone();
                        recipe_state.set(edited_recipe);
                    })
                }
                close={
                    Callback::from(move|_| {
                        edit_mode.set(false);
                        mode_state.set(RecipeComponentMode::View)
                    }
            )}/>}
        }
            RecipeComponentMode::New=>{
                html!{
                <NewRecipe
                full_recipe={full_recipe.clone()}
                new_recipe_cb={
                    Callback::from(move |edited_recipe:FullRecipe|{
                        let recipe_state = recipe_state.clone();
                        recipe_state.set(edited_recipe);
                    })
                }
                />
            }
        },RecipeComponentMode::View=>{html!()}
        }}


    </>
    }
}

#[derive(Properties, PartialEq)]
pub struct StepListProps {
    pub step_list: Vec<Step>,
    #[prop_or_default]
    pub edit_mode:bool,
    #[prop_or_default]
    pub curr_focus:Callback<Step>
}
#[function_component(StepList)]
pub fn step_list(props: &StepListProps) -> Html {
    let StepListProps { step_list, edit_mode, curr_focus }=props;

    let l: Vec<Html> = step_list
        .iter()
        .map(|step: &Step| {
            // note: calling unwrap on step.id because step will always receive an id
            html! {
                <li id={format!("step-{}",step.id.unwrap_or(props.step_list.len().try_into().expect("invalid len()")))} class="step">
                    <StepItem 
                        {edit_mode} 
                        {curr_focus}
                        step={step.clone()}/>
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
    #[prop_or_default]
    pub edit_mode:bool,
    #[prop_or_default]
    pub curr_focus:Callback<Ingredient>
}
#[function_component(IngredientList)]
/// Represents a list of ingredients
///
/// Uses a vec![Ingredient] as a prop.
pub fn ingredients_list(props: &IngredientsListProps) -> Html {
    let IngredientsListProps { 
        ingredients,
        curr_focus, 
        edit_mode 
    } = props;

    let ingredient_list: Vec<Html> = ingredients
        .iter()
        .map(|ingredient| {  
            html! {
            <>
                <li id={format!{"ingredient-{}",ingredient.id.unwrap_or(ingredients.len().try_into().expect("invalid len()"))}}>
                
                    <IngredientItem 
                        {edit_mode} 
                        {curr_focus}
                        ingredient={ingredient.clone()} 
                        />
                </li>
                    </>
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
