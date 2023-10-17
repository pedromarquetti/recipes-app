mod recipe_route;
mod step_route;
mod user_route;

use crate::routes::{recipe_route::create_recipe, step_route::create_step};
use db::db_pool::Pool;
use warp::{http::method::Method, path, Filter, Rejection, Reply};

use self::{
    recipe_route::{delete_recipe, fuzzy_query_recipe, view_recipe},
    step_route::delete_step,
    user_route::{create_user, delete_user},
};

pub fn routing_table(pool: Pool) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
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

    // API endpoints

    //recipe endpoints
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

    // recipe step endpoints
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

    create_recipe
        .or(create_recipe_step)
        .or(fuzzy_query)
        .or(delete_recipe)
        .or(delete_recipe_step)
        .or(create_user)
        .or(delete_user)
        .or(view_recipe)
        .with(cors)
}
