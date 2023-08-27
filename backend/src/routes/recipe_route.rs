use diesel::{ExpressionMethods, RunQueryDsl};
use log::{debug, info};
use serde_json::json;
use warp::{reply::Response, Rejection, Reply};

use crate::{
    db::{recipe::Recipe, DbConnection},
    error::{convert_to_rejection, Error},
    schema::recipe_step::recipe_id,
};

pub async fn create_recipe(
    db_connection: DbConnection,
    recipe: Recipe,
) -> Result<impl Reply, Rejection> {
    use crate::schema::recipe;
    debug!("{:?}", recipe);

    let mut conn = db_connection.map_err(convert_to_rejection)?;

    let id = diesel::insert_into(recipe::table)
        .values::<Recipe>(recipe)
        .returning(recipe::id)
        .execute(&mut conn)
        .map_err(convert_to_rejection)?;

    Ok(warp::reply::json(&json!({ "msg": format!("{:?}", id) })))
}

pub async fn delete_recipe(
    db_connection: DbConnection,
    incoming_recipe: Recipe,
) -> Result<impl Reply, Rejection> {
    let mut conn = db_connection.map_err(convert_to_rejection)?;
    use crate::schema::recipe;
    match incoming_recipe.id {
        Some(incoming_id) => {
            diesel::delete(recipe::table)
                .filter(recipe::id.eq(incoming_id))
                .execute(&mut conn)
                .map_err(convert_to_rejection)?;
            Ok(warp::reply::json(&json!({
                "msg": format!("recipe {} deleted", incoming_recipe.recipe_name)
            })))
        }
        None => return Err(Error::payload_error("must specify recipe id to delete").into()),
    }
}
