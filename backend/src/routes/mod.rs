pub mod auth;
pub mod ingredient_route;
pub mod recipe_route;
pub mod step_route;
pub mod user_route;

use self::{
    ingredient_route::{create_ingredient, delete_ingredient, update_ingredient},
    recipe_route::{delete_recipe, fuzzy_query_recipe, update_recipe, view_recipe},
    step_route::{delete_step, update_step},
    user_route::{
        create_user, delete_user, get_user_name, list_users, login_user_route,
        update_user_info_route,
    },
};
use crate::routes::auth::auth;
use crate::{
    jwt::UserClaims,
    routes::{recipe_route::create_recipe, step_route::create_step},
};
use db::{
    db_pool::Pool,
    structs::{FullRecipe, Ingredient, Step, UrlRecipeQuery, UrlUserQuery, UserRole},
};
use log::debug;
use serde_json::json;
use warp::{http::method::Method, path, Filter, Rejection, Reply};

pub fn routing_table(pool: Pool) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    // this filter will be used to get a valid connection to the db pool
    let pool_filter = warp::any().map(move || pool.get());

    // setting up CORS
    // these settings will be ALLOWED by the server so the client knows what the backend accept
    let cors = warp::cors()
        .allow_headers(vec![
            // list of headers the server will allow in the request body
            // "content-type" used for JSON requests from the frontend
            "content-type",
        ])
        // allowing methods that will be used by/allowed to the client
        .allow_methods(vec![Method::POST, Method::GET])
        // from my understanding, since this is a public API, I can allow any origin here
        .allow_any_origin();

    /* list of API endpoints */

    // recipe endpoints
    let create_recipe = warp::post()
        .and(path!("api" / "create" / "recipe"))
        .and(warp::body::json())
        .and(auth())
        .and(pool_filter.clone())
        .and_then(create_recipe);
    let delete_recipe = warp::get()
        .and(path!("api" / "delete" / "recipe"))
        .and(warp::query::<UrlRecipeQuery>())
        .and(auth())
        .and(pool_filter.clone())
        .and_then(delete_recipe);
    let view_recipe = warp::get()
        .and(path!("api" / "get" / "recipe"))
        .and(warp::query::<UrlRecipeQuery>())
        .and(pool_filter.clone())
        .and_then(view_recipe);
    let fuzzy_query = warp::get()
        .and(path!("api" / "get" / "recipes"))
        .and(warp::query::<UrlRecipeQuery>())
        .and(pool_filter.clone())
        .and_then(fuzzy_query_recipe);
    let update_recipe = warp::post()
        .and(path!("api" / "update" / "recipe"))
        .and(warp::body::json())
        .and(auth())
        .and(pool_filter.clone())
        .and_then(update_recipe);

    //  step endpoints
    let create_recipe_step = warp::post()
        .and(path!("api" / "create" / "step"))
        .and(pool_filter.clone())
        .and(warp::body::json())
        .and(auth())
        .and_then(create_step);
    let delete_recipe_step = warp::post()
        .and(path!("api" / "delete" / "step"))
        .and(warp::query::<Step>())
        .and(auth())
        .and(pool_filter.clone())
        .and_then(delete_step);
    let update_recipe_step = warp::post()
        .and(path!("api" / "update" / "step"))
        .and(pool_filter.clone())
        .and(warp::body::json())
        .and(auth())
        .and_then(update_step);

    //  ingredient endpoits
    let create_recipe_ingredient = warp::post()
        .and(path!("api" / "create" / "ingredient"))
        .and(pool_filter.clone())
        .and(warp::body::json())
        .and(auth())
        .and_then(create_ingredient);
    let delete_recipe_ingredient = warp::post()
        .and(path!("api" / "delete" / "ingredient"))
        .and(warp::body::json::<Ingredient>())
        .and(auth())
        .and(pool_filter.clone())
        .and_then(delete_ingredient);
    let update_recipe_ingredient = warp::post()
        .and(path!("api" / "update" / "ingredient"))
        .and(pool_filter.clone())
        .and(warp::body::json())
        .and(auth())
        .and_then(update_ingredient);

    // user endpoints
    let create_user = warp::post()
        .and(path!("api" / "create" / "user"))
        .and(pool_filter.clone())
        .and(warp::body::json())
        .and_then(create_user);
    let delete_user = warp::get()
        .and(path!("api" / "delete" / "user"))
        .and(warp::query::<UrlUserQuery>())
        .and(auth())
        .and(pool_filter.clone())
        .and_then(delete_user);
    let get_user_info = warp::get()
        .and(path!("api" / "get" / "username"))
        .and(warp::query::<UrlUserQuery>())
        .and(auth())
        .and(pool_filter.clone())
        .and_then(get_user_name);
    let login_user = warp::post()
        .and(path!("api" / "login" / "user"))
        .and(pool_filter.clone())
        .and(warp::body::json())
        .and_then(login_user_route);
    let update_user = warp::post()
        .and(path!("api" / "login" / "user"))
        .and(pool_filter.clone())
        .and(warp::body::json())
        .and(auth())
        .and_then(update_user_info_route);
    let list_users = warp::get()
        .and(path!("api" / "list" / "user"))
        .and(pool_filter.clone())
        .and(auth())
        .and_then(list_users);

    let ping_endpoint = warp::any().and(path!("api" / "ping")).and_then(ping);

    let user_endpoints = create_user
        .or(get_user_info)
        .or(list_users)
        .or(delete_user)
        .or(login_user)
        .or(update_user);
    let recipe_endpoints = create_recipe
        .or(update_recipe)
        .or(delete_recipe)
        .or(view_recipe)
        .or(fuzzy_query);
    let recipe_step_endpoints = create_recipe_step
        .or(update_recipe_step)
        .or(delete_recipe_step);
    let recipe_ingredient_endpoints = create_recipe_ingredient
        .or(delete_recipe_ingredient)
        .or(update_recipe_ingredient);

    ping_endpoint
        .or(user_endpoints)
        .or(recipe_endpoints)
        .or(recipe_step_endpoints)
        .or(recipe_ingredient_endpoints)
        .with(cors)
}

pub async fn ping() -> Result<impl Reply, Rejection> {
    Ok(warp::reply::json(&json!({"msg":"I'm Here"})))
}

/// checks if current user can create/read/update/delete item
pub fn validate_permission(recipe: FullRecipe, claims: Option<UserClaims>) -> bool {
    if let Some(user_id) = &recipe.recipe.user_id {
        // recipe has owner
        if claims.is_none() || user_id.ne(&claims.unwrap().user_id) {
            // no login token found OR no matchva
            return false;
        } else {
            // user owns recipe
            return true;
        }
    } else {
        // recipe has no owner
        if let Some(claims) = claims {
            if claims.role.eq(&UserRole::Admin) {
                // admins can edit any recipe
                return true;
            }
        }
        // no token found
        return false;
    }
}
