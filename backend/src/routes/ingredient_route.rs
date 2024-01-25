use crate::{
    error::{convert_to_rejection, Error},
    jwt::UserClaims,
};
use db::{
    db_pool::DbConnection,
    functions::{recipe::query_full_recipe, recipe_ingredient::*},
    structs::{Ingredient, Recipe},
};
use serde_json::json;
use warp::{http::StatusCode, Rejection, Reply};

use super::validate_permission;

/// Backend ingredient creator endpoint function
pub async fn create_ingredient(
    db_conn: DbConnection,
    ingredients: Vec<Ingredient>,
    claims: Option<UserClaims>,
) -> Result<impl Reply, Rejection> {
    let mut conn = db_conn.map_err(convert_to_rejection)?;
    let mut r = Recipe::new();
    r.set_id(ingredients[0].recipe_id);
    let recipe = query_full_recipe(&mut conn, &r).map_err(convert_to_rejection)?;

    if validate_permission(recipe, claims) {
        create_recipe_ingredient_query(conn, &ingredients).map_err(convert_to_rejection)?;
        return Ok(warp::reply::json(
            &json!({"msg":format!("created {} ingredients",ingredients.len())}),
        ));
    }
    return Err(Error::user_error("Cannot create ingredient!", StatusCode::FORBIDDEN).into());
}

/// Backend ingredient updater endpoint function
pub async fn update_ingredient(
    db_conn: DbConnection,
    ingredient: Ingredient,
    claims: Option<UserClaims>,
) -> Result<impl Reply, Rejection> {
    if ingredient.id.is_none() {
        return Err(Error::payload_error("missing ingredient ID field").into());
    }

    let mut conn = db_conn.map_err(convert_to_rejection)?;
    let mut r = Recipe::new();
    r.set_id(ingredient.recipe_id);
    let recipe = query_full_recipe(&mut conn, &r).map_err(convert_to_rejection)?;

    if validate_permission(recipe, claims) {
        update_ingredient_query(conn, &ingredient).map_err(convert_to_rejection)?;
        return Ok(warp::reply::json(
            &json!({"msg":format!("Ingredient {} deleted",ingredient.ingredient_name)}),
        ));
    }
    return Err(Error::user_error("Cannot update ingredient!", StatusCode::FORBIDDEN).into());
}

/// Backend ingredient delete endpoint function
pub async fn delete_ingredient(
    db_conn: DbConnection,
    ingredient: Ingredient,
    claims: Option<UserClaims>,
) -> Result<impl Reply, Rejection> {
    if ingredient.id.is_none() {
        return Err(Error::payload_error("missing ingredient ID field").into());
    }

    let mut conn = db_conn.map_err(convert_to_rejection)?;
    let mut r = Recipe::new();
    r.set_id(ingredient.recipe_id);
    let recipe = query_full_recipe(&mut conn, &r).map_err(convert_to_rejection)?;

    if validate_permission(recipe, claims) {
        delete_recipe_ingredient_query(conn, &ingredient).map_err(convert_to_rejection)?;
        return Ok(warp::reply::json(
            &json!({"msg":format!("Ingredient {} deleted",ingredient.ingredient_name)}),
        ));
    }
    return Err(Error::user_error("Cannot delete ingredient!", StatusCode::FORBIDDEN).into());
}
