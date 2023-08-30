use diesel::{ExpressionMethods, RunQueryDsl};
use serde_json::json;
use warp::{Rejection, Reply};

use crate::{
    db::{recipe::Recipe, DbConnection, OkPool},
    error::{convert_to_rejection, Error},
};

pub async fn create_recipe(
    db_connection: DbConnection,
    recipe: Recipe,
) -> Result<impl Reply, Rejection> {
    use crate::schema::recipe;

    let mut conn: OkPool = db_connection.map_err(convert_to_rejection)?;

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
    let mut conn: OkPool = db_connection.map_err(convert_to_rejection)?;
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
