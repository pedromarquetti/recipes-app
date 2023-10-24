use crate::error::{convert_to_rejection, Error};
use db::{db_pool::DbConnection, functions::recipe_ingredient::*, structs::Ingredient};
use serde_json::json;
use warp::{Rejection, Reply};

/// Backend ingredient creator endpoint function
pub async fn create_ingredient(
    db_conn: DbConnection,
    ingredients: Vec<Ingredient>,
) -> Result<impl Reply, Rejection> {
    let conn = db_conn.map_err(convert_to_rejection)?;
    create_recipe_ingredient_record(conn, &ingredients).map_err(convert_to_rejection)?;
    Ok(warp::reply::json(
        &json!({"msg":format!("created {} ingredients",ingredients.len())}),
    ))
}

/// Backend ingredient updater endpoint function
pub async fn update_ingredient(
    db_conn: DbConnection,
    ingredients: Ingredient,
) -> Result<impl Reply, Rejection> {
    if ingredients.id.is_none() {
        return Err(Error::payload_error("missing ingredient ID field").into());
    }

    let conn = db_conn.map_err(convert_to_rejection)?;
    update_ingredient_query(conn, &ingredients).map_err(convert_to_rejection)?;
    Ok(warp::reply::json(
        &json!({"msg":format!("Ingredient {} deleted",ingredients.ingredient_name)}),
    ))
}

/// Backend ingredient delete endpoint function
pub async fn delete_ingredient(
    db_conn: DbConnection,
    ingredients: Ingredient,
) -> Result<impl Reply, Rejection> {
    if ingredients.id.is_none() {
        return Err(Error::payload_error("missing ingredient ID field").into());
    }
    let conn = db_conn.map_err(convert_to_rejection)?;
    delete_recipe_ingredient_record(conn, &ingredients).map_err(convert_to_rejection)?;
    Ok(warp::reply::json(
        &json!({"msg":format!("Ingredient {} deleted",ingredients.ingredient_name)}),
    ))
}
