pub mod error;
pub mod home;
pub mod login;
pub mod recipe;
pub mod recipe_list;
pub mod register;

use recipe::RecipePage;
use yew::prelude::*;
use yew_router::prelude::*;

use self::{
    error::{ErrorPage, ErrorType},
    home::Home,
    login::UserLogin,
    recipe_list::RecipeList,
    register::UserRegister,
};

/// # Routes enum
///
/// Specifies all possible endpoints the user can access
#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/register")]
    Register,
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
        Route::Home => {
            html! {<Home />}
        }
        Route::NotFound => {
            html! {
                <ErrorPage error_type={ErrorType::NotFound} text={"Not Found"}/>
            }
        }
        Route::RecipeList { name } => {
            html! {
            <RecipeList recipe_name={name}/>
            }
        }
        Route::Recipe { id } => {
            html! {
            <RecipePage recipe_id={id}/>
            }
        }
        Route::Login => {
            html! {
                <UserLogin/>
            }
        }
        Route::Register => {
            html! {
                <UserRegister/>
            }
        }
    }
}
