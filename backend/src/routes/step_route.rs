use serde_json::json;
use warp::{http::StatusCode, reject::Rejection, reply::Reply};

use crate::{
    error::{convert_to_rejection, Error},
    jwt::UserClaims,
};
use db::{
    db_pool::{DbConnection, PooledPgConnection},
    functions::{
        recipe::query_full_recipe,
        recipe_step::{update_step_query, *},
    },
    structs::{Recipe, Step, UrlRecipeQuery},
};

use super::validate_permission;

pub async fn create_step(
    db_connection: DbConnection,
    recipe_steps: Vec<Step>,
    user_claims: Option<UserClaims>,
) -> Result<impl Reply, Rejection> {
    let mut conn = db_connection.map_err(convert_to_rejection)?;
    let mut r = Recipe::default();
    r.set_id(recipe_steps[0].recipe_id);
    let recipe = query_full_recipe(
        &mut conn,
        &UrlRecipeQuery {
            id: r.id,
            name: None,
        },
    )
    .map_err(convert_to_rejection)?;

    if validate_permission(recipe.recipe.user_id, user_claims) {
        create_recipe_step_query(conn, &recipe_steps).map_err(convert_to_rejection)?;

        return Ok(warp::reply::json(&json!({
            "msg": format!("{} steps created ", recipe_steps.len())
        })));
    }
    return Err(Error::user_error("Cannot create step!", StatusCode::FORBIDDEN).into());
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
    let mut r = Recipe::default();
    r.set_id(recipe_step.recipe_id);
    let recipe = query_full_recipe(
        &mut conn,
        &UrlRecipeQuery {
            id: r.id,
            name: None,
        },
    )
    .map_err(convert_to_rejection)?;
    if validate_permission(recipe.recipe.user_id, user_claims) {
        update_step_query(conn, &recipe_step).map_err(convert_to_rejection)?;
        return Ok(warp::reply::json(
            &json!({"msg":format!("step {} modified",recipe_step.step_name)}),
        ));
    }
    return Err(Error::user_error("Cannot update step!", StatusCode::FORBIDDEN).into());
}

pub async fn delete_step(
    incoming_query: Step,
    user_claims: Option<UserClaims>,
    db_connection: DbConnection,
) -> Result<impl Reply, Rejection> {
    if incoming_query.id.is_none() {
        return Err(Error::payload_error("name or id must be supplied!").into());
    }

    let mut conn: PooledPgConnection = db_connection.map_err(convert_to_rejection)?;

    let recipe = query_full_recipe(
        &mut conn,
        &UrlRecipeQuery {
            id: Some(incoming_query.recipe_id),
            name: None,
        },
    )
    .map_err(convert_to_rejection)?;
    if validate_permission(recipe.recipe.user_id, user_claims) {
        delete_recipe_step_query(conn, &incoming_query).map_err(convert_to_rejection)?;

        return Ok(warp::reply::json(&json!({
            "msg": format!("step {} deleted", incoming_query.step_name)
        })));
    }
    return Err(Error::user_error("Cannot delete recipe step!", StatusCode::FORBIDDEN).into());
}
