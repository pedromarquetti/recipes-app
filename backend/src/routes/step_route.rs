use serde_json::json;
use warp::{http::StatusCode, reject::Rejection, reply::Reply};

use crate::{
    error::{convert_to_rejection, Error},
    jwt::UserClaims,
};
use db::{
    db_pool::DbConnection,
    functions::{
        recipe::query_full_recipe,
        recipe_step::{update_step_query, *},
    },
    structs::{Recipe, Step, UserRole},
};

pub async fn create_step(
    db_connection: DbConnection,
    recipe_steps: Vec<Step>,
    user_claims: Option<UserClaims>,
) -> Result<impl Reply, Rejection> {
    let mut conn = db_connection.map_err(convert_to_rejection)?;
    let mut r = Recipe::new();
    r.set_id(recipe_steps[0].recipe_id);
    let recipe = query_full_recipe(&mut conn, &r).map_err(convert_to_rejection)?;

    // recipe has owner
    if let Some(user_id) = &recipe.recipe.user_id {
        // no login token found OR no match
        if user_claims.is_none() || user_id.ne(&user_claims.unwrap().user_id) {
            return Err(Error::user_error("Action not allowed!", StatusCode::FORBIDDEN).into());
        } else {
            // user owns recipe
            // running query
            create_recipe_step_query(conn, &recipe_steps).map_err(convert_to_rejection)?;

            Ok(warp::reply::json(&json!({
                "msg": format!("{} steps created ", recipe_steps.len())
            })))
        }
    } else {
        // recipe has no owner
        if let Some(claims) = user_claims {
            if claims.role.eq(&UserRole::Admin) {
                // admins can edit any recipe
                // running query
                create_recipe_step_query(conn, &recipe_steps).map_err(convert_to_rejection)?;

                return Ok(warp::reply::json(&json!({
                    "msg": format!("{} steps created ", recipe_steps.len())
                })));
            }
        }
        // no token found
        return Err(Error::user_error("Recipe cannot be deleted", StatusCode::FORBIDDEN).into());
    }
}

pub async fn update_step(
    db_connection: DbConnection,
    recipe_step: Step,
    user_claims: Option<UserClaims>,
) -> Result<impl Reply, Rejection> {
    if recipe_step.id.is_none() {
        // this should never happen, but adding it here just in case...
        return Err(Error::payload_error("step ID is missing!").into());
    }
    let mut conn = db_connection.map_err(convert_to_rejection)?;
    let mut r = Recipe::new();
    r.set_id(recipe_step.recipe_id);
    let recipe = query_full_recipe(&mut conn, &r).map_err(convert_to_rejection)?;

    // recipe has owner
    if let Some(user_id) = &recipe.recipe.user_id {
        // no login token found OR no match
        if user_claims.is_none() || user_id.ne(&user_claims.unwrap().user_id) {
            return Err(Error::user_error("Action not allowed!", StatusCode::FORBIDDEN).into());
        } else {
            // user owns recipe
            // running query
            update_step_query(conn, &recipe_step).map_err(convert_to_rejection)?;
            Ok(warp::reply::json(
                &json!({"msg":format!("step {} modified",recipe_step.step_name)}),
            ))
        }
    } else {
        // recipe has no owner
        if let Some(claims) = user_claims {
            if claims.role.eq(&UserRole::Admin) {
                // admins can edit any recipe
                // running query
                update_step_query(conn, &recipe_step).map_err(convert_to_rejection)?;
                return Ok(warp::reply::json(
                    &json!({"msg":format!("step {} modified",recipe_step.step_name)}),
                ));
            }
        }
        // no token found
        return Err(Error::user_error("Recipe cannot be deleted", StatusCode::FORBIDDEN).into());
    }
}

pub async fn delete_step(
    db_connection: DbConnection,
    recipe_step: Step,
    user_claims: Option<UserClaims>,
) -> Result<impl Reply, Rejection> {
    if recipe_step.id.is_none() {
        return Err(Error::payload_error("step_id must be specified").into());
    }
    let mut conn = db_connection.map_err(convert_to_rejection)?;
    let mut r = Recipe::new();
    r.set_id(recipe_step.recipe_id);
    let recipe = query_full_recipe(&mut conn, &r).map_err(convert_to_rejection)?;

    // recipe has owner
    if let Some(user_id) = &recipe.recipe.user_id {
        // no login token found OR no match
        if user_claims.is_none() || user_id.ne(&user_claims.unwrap().user_id) {
            return Err(Error::user_error("Action not allowed!", StatusCode::FORBIDDEN).into());
        } else {
            // user owns recipe
            // running query
            // running query
            delete_recipe_step_query(conn, &recipe_step).map_err(convert_to_rejection)?;

            Ok(warp::reply::json(&json!({
                "msg": format!("step {} deleted", recipe_step.step_name)
            })))
        }
    } else {
        // recipe has no owner
        if let Some(claims) = user_claims {
            if claims.role.eq(&UserRole::Admin) {
                // admins can edit any recipe
                // running query
                // running query
                delete_recipe_step_query(conn, &recipe_step).map_err(convert_to_rejection)?;

                return Ok(warp::reply::json(&json!({
                    "msg": format!("step {} deleted", recipe_step.step_name)
                })));
            }
        }
        // no token found
        return Err(Error::user_error("Recipe cannot be deleted", StatusCode::FORBIDDEN).into());
    }
}
