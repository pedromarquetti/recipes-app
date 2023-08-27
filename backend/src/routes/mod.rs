use serde_json::Value;
use warp::{path, Filter, Rejection, Reply};

mod recipe_route;
mod step_route;
mod user_route;

use crate::{
    db::Pool,
    routes::{recipe_route::create_recipe, step_route::create_step},
};

use self::{
    recipe_route::delete_recipe,
    step_route::delete_step,
    user_route::{create_user, delete_user},
};

pub fn routing_table(pool: Pool) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let pool_filter = warp::any().map(move || pool.get());

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
        .or(delete_recipe)
        .or(delete_recipe_step)
        .or(create_user)
        .or(delete_user)
}
