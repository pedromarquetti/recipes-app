use serde_json::json;
use warp::{Rejection, Reply};

use crate::error::{convert_to_rejection, Error};
use db::{
    db_pool::{DbConnection, PooledPgConnection},
    functions::recipe::{
        create_recipe_query, delete_recipe_query, fuzzy_query, query_recipe, update_recipe_query,
    },
    structs::{FullRecipe, Recipe},
};

pub async fn create_recipe(
    db_connection: DbConnection,
    recipe: Recipe,
) -> Result<impl Reply, Rejection> {
    let conn: PooledPgConnection = db_connection.map_err(convert_to_rejection)?;

    let created_recipe = create_recipe_query(conn, &recipe).map_err(convert_to_rejection)?;

    Ok(warp::reply::json(&json!({
        "msg": format!("created recipe {}, with id {}", recipe.recipe_name, created_recipe.id.unwrap())
    })))
}

pub async fn delete_recipe(
    db_connection: DbConnection,
    incoming_recipe: Recipe,
) -> Result<impl Reply, Rejection> {
    let conn: PooledPgConnection = db_connection.map_err(convert_to_rejection)?;
    if incoming_recipe.id.is_none() {
        return Err(Error::payload_error("must specify recipe id to delete").into());
    }
    delete_recipe_query(conn, &incoming_recipe).map_err(convert_to_rejection)?;
    Ok(warp::reply::json(&json!({
        "msg": format!("recipe {} deleted", incoming_recipe.recipe_name)
    })))
}

pub async fn view_recipe(
    db_connection: DbConnection,
    incoming_recipe: Recipe,
) -> Result<impl Reply, Rejection> {
    if incoming_recipe.id.is_some() {
        let conn = db_connection.map_err(convert_to_rejection)?;
        let recipe = query_recipe(conn, &incoming_recipe).map_err(convert_to_rejection)?;

        Ok(warp::reply::json::<FullRecipe>(&recipe))
    } else {
        Err(Error::payload_error("Recipe id not specified!").into())
    }
}
pub async fn fuzzy_query_recipe(
    db_connection: DbConnection,
    incoming_recipe: Recipe,
) -> Result<impl Reply, Rejection> {
    let conn = db_connection.map_err(convert_to_rejection)?;
    let recipe = fuzzy_query(conn, &incoming_recipe).map_err(convert_to_rejection)?;

    Ok(warp::reply::json::<Vec<Recipe>>(&recipe))
}
pub async fn update_recipe(
    db_connection: DbConnection,
    incoming_recipe: Recipe,
) -> Result<impl Reply, Rejection> {
    if incoming_recipe.id.is_none() {
        return Err(Error::payload_error("Recipe ID missing!").into());
    }
    let conn = db_connection.map_err(convert_to_rejection)?;
    let recipe = update_recipe_query(conn, &incoming_recipe).map_err(convert_to_rejection)?;

    Ok(warp::reply::json::<Recipe>(&recipe))
}
