mod home;
pub mod recipe_list_page;
mod recipe_page;

use recipe_page::RecipePage;
use yew::prelude::*;
use yew_router::prelude::*;

use self::home::Home;

/// # Routes enum
///
/// Specifies all possible endpoints the user can access
#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/recipe/:id")]
    Recipe { id: i32 },
    #[at("/recipe/list/:name")]
    RecipeList { name: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

/// # Main switch function
/// Handles switching between routes
///
/// # Arguments
///
/// * `route` - Route enum with possible routes.
pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! {<Home />},
        Route::NotFound => html! {<>{"404"}</>},
        Route::RecipeList { name } => html! {<></>},
        Route::Recipe { id } => {
            html! {
            <RecipePage recipe_id={id}/>
            }
        }
    }
}
