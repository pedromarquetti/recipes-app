use serde_json::json;
use warp::{filters::body::json, http::StatusCode, Rejection, Reply};

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
    structs::{FullRecipe, Recipe, UserRole},
};

pub async fn create_recipe(
    user_claims: Option<UserClaims>,
    db_connection: DbConnection,
    mut recipe: Recipe,
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
        let mut conn = db_connection.map_err(convert_to_rejection)?;
        let recipe =
            query_full_recipe(&mut conn, &incoming_recipe).map_err(convert_to_rejection)?;

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
    user_claims: Option<UserClaims>,
    db_connection: DbConnection,
    incoming_recipe: Recipe,
) -> Result<impl Reply, Rejection> {
    if incoming_recipe.id.is_none() {
        return Err(Error::payload_error("Recipe ID missing!").into());
    }
    let mut conn: PooledPgConnection = db_connection.map_err(convert_to_rejection)?;

    // querying recipe so we can validate ownership
    let recipe = query_full_recipe(&mut conn, &incoming_recipe).map_err(convert_to_rejection)?;

    // if incoming recipe has user_id (owner)
    if let Some(user_id) = &recipe.recipe.user_id {
        // throw error if user doesn't have JWT token OR JWT user id does not match
        if user_claims.is_none() || user_id.ne(&user_claims.expect("expected user claims").user_id)
        {
            return Err(Error::user_error("Action Not allowed", StatusCode::FORBIDDEN).into());
        } else {
            // user owns recipe
            update_recipe_query(conn, &incoming_recipe).map_err(convert_to_rejection)?;

            return Ok(warp::reply::json(&json!({"a":""})));
        }
    } else {
        // recipe has no owner
        // but logged in user tried to edit recipe
        if let Some(claims) = user_claims {
            if claims.role.eq(&UserRole::Admin) {
                // admins can edit any recipe
                update_recipe_query(conn, &incoming_recipe).map_err(convert_to_rejection)?;
                return Ok(warp::reply::json(&json!({"msg":"recipe updated!"})));
            }
        }
        // no user claims and recipe has no owner
        return Err(Error::user_error(
            "Recipe has no owner, can't be updated! You must be an admin to edit this recipe",
            StatusCode::FORBIDDEN,
        )
        .into());
    }
}
