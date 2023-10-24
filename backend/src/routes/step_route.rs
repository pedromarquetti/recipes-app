use serde_json::json;
use warp::{reject::Rejection, reply::Reply};

use crate::error::{convert_to_rejection, Error};
use db::{
    db_pool::DbConnection,
    functions::recipe_step::{update_step_query, *},
    structs::Step,
};

pub async fn create_step(
    db_connection: DbConnection,
    recipe_steps: Vec<Step>,
) -> Result<impl Reply, Rejection> {
    let conn = db_connection.map_err(convert_to_rejection)?;

    // running query
    create_recipe_step_record(conn, &recipe_steps).map_err(convert_to_rejection)?;

    Ok(warp::reply::json(&json!({
        "msg": format!("{} steps created ", recipe_steps.len())
    })))
}

pub async fn update_step(
    db_connection: DbConnection,
    recipe_step: Step,
) -> Result<impl Reply, Rejection> {
    if recipe_step.id.is_none() {
        // this should never happen, but adding it here just in case...
        return Err(Error::payload_error("step ID is missing!").into());
    }
    let conn = db_connection.map_err(convert_to_rejection)?;

    update_step_query(conn, &recipe_step).map_err(convert_to_rejection)?;
    Ok(warp::reply::json(
        &json!({"msg":format!("step {} modified",recipe_step.step_name)}),
    ))
}

pub async fn delete_step(
    db_connection: DbConnection,
    recipe_step: Step,
) -> Result<impl Reply, Rejection> {
    if recipe_step.id.is_none() {
        return Err(Error::payload_error("step_id must be specified").into());
    }
    let conn = db_connection.map_err(convert_to_rejection)?;
    // running query
    delete_recipe_step_record(conn, &recipe_step).map_err(convert_to_rejection)?;

    Ok(warp::reply::json(&json!({
        "msg": format!("step {} deleted", recipe_step.step_name)
    })))
}
