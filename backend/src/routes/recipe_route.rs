use serde_json::json;
use warp::{http::StatusCode, Rejection, Reply};

use crate::{
    error::{convert_to_rejection, Error},
    jwt::UserClaims,
};
use db::{
    db_pool::{DbConnection, PooledPgConnection},
    functions::recipe::{
        create_recipe_query, delete_recipe_query, fuzzy_query, query_full_recipe, query_recipe,
        update_recipe_query,
    },
    structs::{NewRecipe, Recipe, UpdateRecipe, UrlRecipeQuery},
};

use super::validate_permission;

pub async fn create_recipe(
    mut recipe: NewRecipe,
    user_claims: Option<UserClaims>,
    db_connection: DbConnection,
) -> Result<impl Reply, Rejection> {
    let mut conn: PooledPgConnection = db_connection.map_err(convert_to_rejection)?;
    if let Some(claims) = user_claims.clone() {
        recipe.set_user_id(claims.user_id)
    }
    if validate_permission(recipe.user_id, user_claims) {
        return Ok(warp::reply::json(
            // sending query to db
            &create_recipe_query(&mut conn, &recipe).map_err(convert_to_rejection)?,
        ));
    } else {
        return Err(Error::user_error("User not logged in", StatusCode::UNAUTHORIZED).into());
    }
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

    if validate_permission(recipe.recipe.user_id, user_claims) {
        if delete_recipe_query(&mut conn, &incoming_query).map_err(convert_to_rejection)? == 0 {
            return Err(Error::not_found("Recipe not found").into());
        }
        return Ok(warp::reply::json(
            &json!({"msg":format!("recipe {} deleted", recipe.recipe.recipe_name)
            }),
        ));
    } else {
        return Err(Error::user_error("Recipe cannot be deleted", StatusCode::FORBIDDEN).into());
    }
}

pub async fn view_recipe(
    incoming_query: UrlRecipeQuery,
    db_connection: DbConnection,
) -> Result<impl Reply, Rejection> {
    let mut conn = db_connection.map_err(convert_to_rejection)?;
    return Ok(warp::reply::json(
        &query_full_recipe(&mut conn, &incoming_query).map_err(convert_to_rejection)?,
    ));
}

pub async fn fuzzy_query_recipe(
    incoming_query: UrlRecipeQuery,
    db_connection: DbConnection,
) -> Result<impl Reply, Rejection> {
    if incoming_query.name.is_none() {
        return Err(Error::payload_error("name must be supplied!").into());
    }

    let mut conn = db_connection.map_err(convert_to_rejection)?;

    Ok(warp::reply::json::<Vec<Recipe>>(
        &fuzzy_query(&mut conn, &incoming_query.name.unwrap()).map_err(convert_to_rejection)?,
    ))
}

pub async fn update_recipe(
    incoming_recipe: UpdateRecipe,
    user_claims: Option<UserClaims>,
    db_connection: DbConnection,
) -> Result<impl Reply, Rejection> {
    let mut conn: PooledPgConnection = db_connection.map_err(convert_to_rejection)?;

    // querying recipe so we can validate ownership
    let old_recipe = query_recipe(
        &mut conn,
        UrlRecipeQuery {
            id: Some(incoming_recipe.id),
            name: None,
        },
    )
    .map_err(convert_to_rejection)?;

    if validate_permission(old_recipe.user_id, user_claims) {
        let mut updated_recipe = Recipe::default();
        if let Some(recipe_name) = incoming_recipe.recipe_name {
            updated_recipe.recipe_name = recipe_name;
        } else {
            updated_recipe.recipe_name = old_recipe.recipe_name
        }
        updated_recipe.recipe_observations = incoming_recipe.recipe_observations;

        update_recipe_query(&mut conn, &updated_recipe).map_err(convert_to_rejection)?;
        return Ok(warp::reply::json(&json!({"msg":"recipe updated!"})));
    } else {
        return Err(Error::user_error("Cannot update recipe", StatusCode::UNAUTHORIZED).into());
    }
}

pub async fn check_edit_permission(
    incoming_query: UrlRecipeQuery,
    user_claims: Option<UserClaims>,
    db_connection: DbConnection,
) -> Result<impl Reply, Rejection> {
    if incoming_query.id.is_none() {
        return Err(Error::payload_error("Insert a recipe id!").into());
    }
    let mut conn: PooledPgConnection = db_connection.map_err(convert_to_rejection)?;

    // querying recipe so we can validate ownership
    let recipe = query_full_recipe(
        &mut conn,
        &UrlRecipeQuery {
            id: incoming_query.id,
            name: None,
        },
    )
    // returns error if no recipe is found
    .map_err(convert_to_rejection)?;

    if validate_permission(recipe.recipe.user_id, user_claims) {
        return Ok(warp::reply::json(&json!({"msg":"user can edit recipe!"})));
    } else {
        return Err(Error::user_error("Cannot edit recipe", StatusCode::UNAUTHORIZED).into());
    }
}
