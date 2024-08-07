use serde_json::json;
use warp::{http::StatusCode, reject::Rejection, reply::Reply};

use crate::{
    error::{convert_to_rejection, Error},
    jwt::UserClaims,
};
use db::{
    db_pool::{DbConnection, PooledPgConnection},
    functions::{
        recipe::{query_full_recipe, query_recipe},
        recipe_step::{update_step_query, *},
    },
    structs::{NewStep, Recipe, Step, UpdateStep, UrlRecipeQuery},
};

use super::validate_permission;

pub async fn create_step(
    db_connection: DbConnection,
    recipe_steps: Vec<NewStep>,
    user_claims: Option<UserClaims>,
) -> Result<impl Reply, Rejection> {
    let mut conn = db_connection.map_err(convert_to_rejection)?;
    let mut r = Recipe::default();
    r.set_id(recipe_steps[0].recipe_id);
    let recipe = query_full_recipe(
        &mut conn,
        &UrlRecipeQuery {
            id: Some(r.id),
            name: None,
        },
    )
    .map_err(convert_to_rejection)?;

    if validate_permission(recipe.recipe.user_id, user_claims) {
        return Ok(warp::reply::json(&json!(create_step_query(
            &mut conn,
            &recipe_steps
        )
        .map_err(convert_to_rejection)?)));
    }
    return Err(Error::user_error("Cannot create step!", StatusCode::FORBIDDEN).into());
}

pub async fn update_step(
    db_connection: DbConnection,
    input_step: UpdateStep,
    user_claims: Option<UserClaims>,
) -> Result<impl Reply, Rejection> {
    let mut conn = db_connection.map_err(convert_to_rejection)?;
    let r = query_recipe(
        &mut conn,
        UrlRecipeQuery {
            id: Some(input_step.recipe_id),
            name: None,
        },
    )
    .map_err(convert_to_rejection)?;
    let old_step = get_step_detail(&mut conn, input_step.id).map_err(convert_to_rejection)?;
    if validate_permission(r.user_id, user_claims) {
        let mut updated_step = Step::default();
        updated_step.set_recipe_id(old_step.recipe_id);

        if let Some(step_name) = input_step.step_name {
            updated_step.step_name = step_name;
        } else {
            updated_step.step_name = old_step.step_name
        }

        if let Some(step_instruction) = input_step.step_instruction {
            updated_step.step_instruction = step_instruction;
        } else {
            updated_step.step_instruction = old_step.step_instruction
        }

        if let Some(step_duration_min) = input_step.step_duration_min {
            updated_step.step_duration_min = step_duration_min;
        } else {
            updated_step.step_duration_min = old_step.step_duration_min
        }

        let update_query = update_step_query(conn, &updated_step).map_err(convert_to_rejection)?;
        return Ok(warp::reply::json(&json!(update_query)));
    }
    return Err(Error::user_error("Cannot update step!", StatusCode::FORBIDDEN).into());
}

pub async fn delete_step(
    incoming_query: Step,
    user_claims: Option<UserClaims>,
    db_connection: DbConnection,
) -> Result<impl Reply, Rejection> {
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
        if delete_step_query(conn, &incoming_query).map_err(convert_to_rejection)? == 0 {
            return Err(Error::not_found("Step not found").into());
        }

        return Ok(warp::reply::json(&json!({
            "msg": format!("step {} deleted", incoming_query.step_name)
        })));
    }
    return Err(Error::user_error("Cannot delete recipe step!", StatusCode::FORBIDDEN).into());
}
