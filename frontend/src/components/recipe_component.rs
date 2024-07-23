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

use super::{ItemProps, RecipeMode};

#[derive(Properties, PartialEq)]
pub struct RecipeProps {
    pub full_recipe: FullRecipe,
    #[prop_or(RecipeMode::View)]
    pub mode:RecipeMode
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
    
    // let full_recipe = (*recipe_state).clone();
    let full_recipe = recipe_state.clone();
    

    let recipe = full_recipe.recipe.clone();
    let ingredients = full_recipe.ingredients.clone();
    let steps = full_recipe.steps.clone();

    let onclick = {
        let edit_mode = edit_mode.clone();
        let use_notification = use_notification.clone();

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
                            mode_state.set(RecipeMode::Edit);

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
            if RecipeMode::View  == (*mode_state).clone()|| RecipeMode::Edit  == (*mode_state).clone(){
                html!{<button {onclick}>{"Edit Recipe"}</button>}
            } else{html!{}}
        }
            <RecipeTitle owner={full_recipe.recipe_owner_name.clone()} title={recipe.recipe_name}/>

            <IngredientList 
            curr_focus={{
                let ingredient_to_edit = ingredient_to_edit.clone();
                let recipe_state = recipe_state.clone();
                let use_notification = use_notification.clone();

                Callback::from(move |(mode,ingredient)|{
                    match mode {
                        RecipeMode::Edit=>{
                            ingredient_to_edit.set(ingredient)

                        },
                        RecipeMode::Delete=>{
                            let recipe_state = recipe_state.clone();
                            let use_notification = use_notification.clone();
                            let mut recipe = (*recipe_state).clone();
                            let recipe = recipe.remove_ingredient(ingredient.id.unwrap_or(-1));
                            match recipe {
                                Ok(i)=>{
                                    recipe_state.set(FullRecipe { 
                                        ingredients:i,
                                        ..(*recipe_state).clone()
                                    })
                                }
                                Err(err)=>{
                                    use_notification.spawn(Notification::new(
                                    yew_notifications::NotificationType::Error,
                                    "Error!",
                                    err.to_string(),
                                    DEFAULT_NOTIFICATION_DURATION,
                                    ));
                                }
                            }

                        }
                        _=>{}
                    }
                })
            }} 
            mode={(*mode_state).clone()} 
            item_list={ingredients}
            />
            
            <StepList 
            curr_focus={{
                let step_to_edit=step_to_edit.clone();
                let recipe_state = recipe_state.clone();
                let use_notification = use_notification.clone();                Callback::from(move |(mode,step)|{
                match mode {
                    RecipeMode::Edit=>{
                        step_to_edit.set(step)
                    }
                    RecipeMode::Delete=>{
                        let recipe_state = recipe_state.clone();
                        let use_notification = use_notification.clone();
                        let mut recipe = (*recipe_state).clone();
                        let recipe = recipe.remove_step(step.id.unwrap_or(-1));
                        match recipe {
                            Ok(s)=>{
                                recipe_state.set(FullRecipe {
                                    steps:s,
                                    ..(*recipe_state).clone()
                                    });

                            }
                            Err(err)=>{
                                use_notification.spawn(Notification::new(
                                yew_notifications::NotificationType::Error,
                                "Error!",
                                err.to_string(),
                                DEFAULT_NOTIFICATION_DURATION,
                                ));
                            }
                        }
                    }
                    _=>{}
                }
            })}}
            mode={(*mode_state).clone()}
            item_list={steps}
            />
        </div>

        {
        match *mode_state {
            RecipeMode::Edit=>{
                html!{
                <EditRecipe
                full_recipe={(*full_recipe).clone()}
                current_focus={(*ingredient_to_edit).clone()}
                step_to_edit={(*step_to_edit).clone()}
                edited_recipe={
                    Callback::from(move |edited_recipe:FullRecipe|{
                        let recipe_state = recipe_state.clone();
                        recipe_state.set(edited_recipe);
                    })
                }
                close={
                    Callback::from(move|_| {
                        edit_mode.set(false);
                        mode_state.set(RecipeMode::View)
                    }
            )}/>}
        }
            RecipeMode::New=>{
                html!{
                <NewRecipe
                full_recipe={(*full_recipe).clone()}
                new_recipe_cb={
                    Callback::from(move |edited_recipe:FullRecipe|{
                        let recipe_state = recipe_state.clone();
                        recipe_state.set(edited_recipe);
                    })
                }
                />
            }
        },_=>{html!()}
        }}


    </>
    }
}

#[function_component(StepList)]
pub fn step_list(props: &ItemProps<Step>) -> Html {
    
    let ItemProps { 
        item_list,
        curr_focus, 
        mode,
        item :_
    } = props;
    let l: Vec<Html> = item_list
        .iter()
        .map(|step: &Step| {
            let mode = mode.clone();
            // note: calling unwrap on step.id because step will always receive an id
            html! {
                <li id={
                    format!("step-{}",step.id.unwrap_or(item_list.len().try_into().expect("invalid len()")))
                } class="step">
                    <StepItem 
                        {mode} 
                        {curr_focus}
                        item={step.clone()}/>
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

#[function_component(IngredientList)]
/// Represents a list of ingredients
///
/// Uses a vec![Ingredient] as a prop.
pub fn ingredients_list(props: &ItemProps<Ingredient>) -> Html {
    let ItemProps { 
        item_list,
        curr_focus, 
        mode,
        item :_
    } = props;

    let ingredient_list: Vec<Html> = item_list
        .iter()
        .map(|ingredient| {  
            let mode= mode.clone();
            let item = ingredient.clone();
            html! {
            <>
                <li id={format!{"ingredient-{}",ingredient.id.unwrap_or(item_list.len().try_into().expect("invalid len()"))}}>
                
                    <IngredientItem 
                        {mode} 
                        {curr_focus}
                        {item} 
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
