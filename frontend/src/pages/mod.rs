mod recipe_page;

use recipe_page::RecipePage;
use yew::prelude::*;
use yew_router::prelude::*;

/// # Routes enum
///
/// Specifies all possible endpoints the user can access
#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/recipe/:id")]
    Recipe { id: i32 },
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
        Route::Home => html! {<>{"Home"}</>},
        Route::NotFound => html! {<>{"404"}</>},
        Route::Recipe { id } => {
            html! {
            <RecipePage recipe_id={id}/>
            }
        }
    }
}
