use diesel::{ExpressionMethods, RunQueryDsl};
use serde_json::json;
use warp::{reject::Rejection, reply::Reply};

use crate::{
    db::{recipe::Step, DbConnection},
    error::{convert_to_rejection, Error},
};

pub async fn create_step(
    db_connection: DbConnection,
    recipe_steps: Vec<Step>,
) -> Result<impl Reply, Rejection> {
    use crate::schema::recipe_step;
    let mut conn = db_connection.map_err(convert_to_rejection)?;

    diesel::insert_into(recipe_step::table)
        .values(&recipe_steps)
        .execute(&mut conn)
        .map_err(convert_to_rejection)?;

    Ok(warp::reply::json(&json!({
        "msg": format!("{} steps creates ", recipe_steps.len())
    })))
}
pub async fn delete_step(
    db_connection: DbConnection,
    recipe_step: Step,
) -> Result<impl Reply, Rejection> {
    use crate::schema::recipe_step;

    let mut conn = db_connection.map_err(convert_to_rejection)?;
    match recipe_step.id {
        Some(ok_step) => {
            diesel::delete(recipe_step::table)
                .filter(recipe_step::id.eq(ok_step))
                .execute(&mut conn)
                .map_err(convert_to_rejection)?;
            Ok(warp::reply::json(&json!({
                "msg": format!("step {} deleted", recipe_step.step_name)
            })))
        }
        None => return Err(Error::payload_error("step_id must be specified").into()),
    }
}
