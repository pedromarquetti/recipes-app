mod ingredient_route;
mod recipe_route;
mod step_route;
mod user_route;

use crate::routes::{recipe_route::create_recipe, step_route::create_step};
use db::db_pool::Pool;
use warp::{http::method::Method, path, Filter, Rejection, Reply};

use self::{
    ingredient_route::{create_ingredient, delete_ingredient, update_ingredient},
    recipe_route::{delete_recipe, fuzzy_query_recipe, update_recipe, view_recipe},
    step_route::{delete_step, update_step},
    user_route::{create_user, delete_user, get_user_info},
};

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
        .allow_methods(vec![Method::POST])
        // from my understanding, since this is a public API, I can allow any origin here
        .allow_any_origin();

    /* list of API endpoints */

    // recipe endpoints
    let create_recipe = warp::post()
        .and(warp::body::content_length_limit(1024 * 10))
        .and(path!("api" / "create" / "recipe"))
        .and(pool_filter.clone())
        .and(warp::body::json())
        .and_then(create_recipe);
    let delete_recipe = warp::post()
        .and(warp::body::content_length_limit(1024 * 10))
        .and(path!("api" / "delete" / "recipe"))
        .and(pool_filter.clone())
        .and(warp::body::json())
        .and_then(delete_recipe);
    let view_recipe = warp::post()
        .and(warp::body::content_length_limit(1024 * 10))
        .and(path!("api" / "view" / "recipe"))
        .and(pool_filter.clone())
        .and(warp::body::json())
        .and_then(view_recipe);
    let fuzzy_query = warp::post()
        .and(warp::body::content_length_limit(1024 * 10))
        .and(path!("api" / "get" / "recipes"))
        .and(pool_filter.clone())
        .and(warp::body::json())
        .and_then(fuzzy_query_recipe);
    let update_recipe = warp::post()
        .and(warp::body::content_length_limit(1024 * 10))
        .and(path!("api" / "update" / "recipe"))
        .and(pool_filter.clone())
        .and(warp::body::json())
        .and_then(update_recipe);

    //  step endpoints
    let create_recipe_step = warp::post()
        .and(warp::body::content_length_limit(1024 * 10))
        .and(path!("api" / "create" / "step"))
        .and(pool_filter.clone())
        .and(warp::body::json())
        .and_then(create_step);
    let delete_recipe_step = warp::post()
        .and(warp::body::content_length_limit(1024 * 10))
        .and(path!("api" / "delete" / "step"))
        .and(pool_filter.clone())
        .and(warp::body::json())
        .and_then(delete_step);
    let update_recipe_step = warp::post()
        .and(warp::body::content_length_limit(1024 * 10))
        .and(path!("api" / "update" / "step"))
        .and(pool_filter.clone())
        .and(warp::body::json())
        .and_then(update_step);

    //  ingredient endpoits
    let create_recipe_ingredient = warp::post()
        .and(warp::body::content_length_limit(1024 * 10))
        .and(path!("api" / "create" / "ingredient"))
        .and(pool_filter.clone())
        .and(warp::body::json())
        .and_then(create_ingredient);
    let delete_recipe_ingredient = warp::post()
        .and(warp::body::content_length_limit(1024 * 10))
        .and(path!("api" / "delete" / "ingredient"))
        .and(pool_filter.clone())
        .and(warp::body::json())
        .and_then(delete_ingredient);
    let update_recipe_ingredient = warp::post()
        .and(warp::body::content_length_limit(1024 * 10))
        .and(path!("api" / "update" / "ingredient"))
        .and(pool_filter.clone())
        .and(warp::body::json())
        .and_then(update_ingredient);

    // user endpoints
    let create_user = warp::post()
        .and(warp::body::content_length_limit(1024 * 10))
        .and(path!("api" / "create" / "user"))
        .and(pool_filter.clone())
        .and(warp::body::json())
        .and_then(create_user);
    let delete_user = warp::post()
        .and(warp::body::content_length_limit(1024 * 10))
        .and(path!("api" / "delete" / "user"))
        .and(pool_filter.clone())
        .and(warp::body::json())
        .and_then(delete_user);
    let get_user_info = warp::post()
        .and(warp::body::content_length_limit(1024 * 10))
        .and(path!("api" / "get" / "username"))
        .and(pool_filter.clone())
        .and(warp::body::json())
        .and_then(get_user_info);

    let user_endpoints = create_user.or(get_user_info).or(delete_user);
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

    user_endpoints
        .or(recipe_endpoints)
        .or(recipe_step_endpoints)
        .or(recipe_ingredient_endpoints)
        .with(cors)
}
