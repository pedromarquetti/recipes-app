use log::error;
use yew::{platform::spawn_local, prelude::*};

use crate::{
    components::recipe_card_component::RecipeCard, functions::recipe_functions::fuzzy_list_recipe,
};

// use time::Duration;
// use yew_notifications::{
//     use_notification, Notification, NotificationFactory, NotificationType, NotificationsPosition,
// };

#[derive(Properties, PartialEq)]
pub struct RecipeListProps {
    pub recipe_name: String,
}

/// # Recipe list view
///
/// Iterates through provided recipe list and displays them.
///
/// Results from Home search bar
#[function_component(RecipeList)]
pub fn recipe_list(RecipeListProps { recipe_name }: &RecipeListProps) -> Html {
    let name = recipe_name.clone();
    let recipe_state = use_state(|| vec![]);
    let fetch_msg = use_state(|| String::new());

    // let notifications_manager = use_notification::<Notification>();

    {
        let recipe_state = recipe_state.clone();
        use_effect_with(recipe_state.clone(), move |_| {
            spawn_local(async move {
                match fuzzy_list_recipe(name).await {
                    Ok(ok_recipes) => {
                        recipe_state.set(ok_recipes);
                    }
                    Err(err) => {
                        error!("err {}", err.to_string());
                    }
                }
            });
        })
    }

    let list: Html = recipe_state
        .iter()
        .map(|recipe| {
            let id = recipe.id.unwrap();
            html! {

                <li id={format!("{}",id)}>
                    <RecipeCard recipe={recipe.clone()}/>
                </li>
            }
        })
        .collect();
    html! {

            {
                if recipe_state.len() ==0 {
                    html! {
                        <h1>{"No recipes found!"}</h1>
                    }
                }
                else {
                    html! {
                    <>
                        <h1>{format!("Found {} recipes",recipe_state.clone().len())}</h1>

                    <div class="recipe-card-container">
                        <ul class="recipes-list">
                        {list}
                        </ul>
                    </div>
                    </>

                    }
                }
                }
    }
}
