use log::debug;
use percent_encoding::percent_decode_str;
use serde_json::json;
use warp::{http::StatusCode, Rejection, Reply};

use crate::{
    error::{convert_to_rejection, Error},
    jwt::UserClaims,
};
use db::{
    db_pool::{DbConnection, PooledPgConnection},
    functions::recipe::{
        create_recipe_query, delete_recipe_query, fuzzy_query, query_full_recipe,
        update_recipe_query,
    },
    structs::{Recipe, UrlRecipeQuery},
};

use super::validate_permission;

pub async fn create_recipe(
    mut recipe: Recipe,
    user_claims: Option<UserClaims>,
    db_connection: DbConnection,
) -> Result<impl Reply, Rejection> {
    let conn: PooledPgConnection = db_connection.map_err(convert_to_rejection)?;

    // if user is logged in...
    if let Some(claims) = user_claims {
        // set user_id
        recipe.set_user_id(claims.user_id)
    }

    // sending query to db
    let created_recipe = create_recipe_query(conn, &recipe).map_err(convert_to_rejection)?;

    Ok(warp::reply::json(&created_recipe))
}

pub async fn delete_recipe(
    incoming_query: UrlRecipeQuery,
    user_claims: Option<UserClaims>,
    db_connection: DbConnection,
) -> Result<impl Reply, Rejection> {
    if incoming_query.id.is_none() && incoming_query.name.is_none() {
        return Err(Error::payload_error("name or id must be supplied!").into());
    }

    let mut conn: PooledPgConnection = db_connection.map_err(convert_to_rejection)?;

    let recipe = query_full_recipe(&mut conn, &incoming_query).map_err(convert_to_rejection)?;

    if validate_permission(recipe, user_claims) {
        delete_recipe_query(conn, &incoming_query).map_err(convert_to_rejection)?;
        return Ok(warp::reply::json(&json!({"msg":"recipe deleted"})));
    } else {
        return Err(Error::user_error("Recipe cannot be deleted", StatusCode::FORBIDDEN).into());
    }
}

pub async fn view_recipe(
    incoming_query: UrlRecipeQuery,
    db_connection: DbConnection,
) -> Result<impl Reply, Rejection> {
    debug!("{:?}", incoming_query);
    if incoming_query.id.is_none() && incoming_query.name.is_none() {
        return Err(Error::payload_error("name or id must be supplied!").into());
    }
    let mut conn = db_connection.map_err(convert_to_rejection)?;

    return Ok(warp::reply::json(&json!(
            {"msg":query_full_recipe(&mut conn, &incoming_query).map_err(convert_to_rejection)?
        }
    )));
}

pub async fn fuzzy_query_recipe(
    incoming_query: UrlRecipeQuery,
    db_connection: DbConnection,
) -> Result<impl Reply, Rejection> {
    debug!("{:?}", incoming_query);
    if incoming_query.name.is_none() {
        return Err(Error::payload_error("name must be supplied!").into());
    }

    let conn = db_connection.map_err(convert_to_rejection)?;

    Ok(warp::reply::json::<Vec<Recipe>>(
        &fuzzy_query(conn, &incoming_query.name.unwrap()).map_err(convert_to_rejection)?,
    ))
}

pub async fn update_recipe(
    incoming_recipe: Recipe,
    user_claims: Option<UserClaims>,
    db_connection: DbConnection,
) -> Result<impl Reply, Rejection> {
    if incoming_recipe.id.is_none() {
        return Err(Error::payload_error("Insert a recipe name!").into());
    }
    let mut conn: PooledPgConnection = db_connection.map_err(convert_to_rejection)?;

    // querying recipe so we can validate ownership
    let recipe = query_full_recipe(
        &mut conn,
        &UrlRecipeQuery {
            id: incoming_recipe.id,
            name: None,
        },
    )
    .map_err(convert_to_rejection)?;

    if validate_permission(recipe, user_claims) {
        update_recipe_query(conn, &incoming_recipe).map_err(convert_to_rejection)?;
        return Ok(warp::reply::json(&json!({"msg":"recipe updated!"})));
    } else {
        return Err(Error::user_error("Cannot update recipe", StatusCode::UNAUTHORIZED).into());
    }
}
