use crate::{
    error::{convert_to_rejection, Error},
    jwt::UserClaims,
};
use db::{
    db_pool::{DbConnection, PooledPgConnection},
    functions::{
        recipe::{query_full_recipe, query_recipe},
        recipe_ingredient::*,
    },
    structs::{Ingredient, NewIngredient, Recipe, UpdateIngredient, UrlRecipeQuery},
};
use serde_json::json;
use warp::{http::StatusCode, Rejection, Reply};

use super::validate_permission;

/// Backend ingredient creator endpoint function
pub async fn create_ingredient(
    db_conn: DbConnection,
    ingredients: Vec<NewIngredient>,
    claims: Option<UserClaims>,
) -> Result<impl Reply, Rejection> {
    let mut conn = db_conn.map_err(convert_to_rejection)?;

    let mut r = Recipe::default();
    r.set_id(ingredients[0].recipe_id);
    let recipe = query_full_recipe(
        &mut conn,
        &UrlRecipeQuery {
            id: Some(r.id),
            name: None,
        },
    )
    .map_err(convert_to_rejection)?;

    if validate_permission(recipe.recipe.user_id, claims) {
        return Ok(warp::reply::json(&json!(create_ingredient_query(
            conn,
            &ingredients
        )
        .map_err(convert_to_rejection)?)));
    }
    return Err(Error::user_error("Cannot create ingredient!", StatusCode::FORBIDDEN).into());
}

/// Backend ingredient updater endpoint function
pub async fn update_ingredient(
    db_connection: DbConnection,
    input_ingredient: UpdateIngredient,
    claims: Option<UserClaims>,
) -> Result<impl Reply, Rejection> {
    let mut conn = db_connection.map_err(convert_to_rejection)?;

    let r = query_recipe(
        &mut conn,
        UrlRecipeQuery {
            id: Some(input_ingredient.recipe_id),
            name: None,
        },
    )
    .map_err(convert_to_rejection)?;

    let old_ingredient =
        get_ingredient_detail(&mut conn, input_ingredient.id).map_err(convert_to_rejection)?;

    if validate_permission(r.user_id, claims) {
        let mut new_ingredient = Ingredient::default();
        new_ingredient.set_recipe_id(old_ingredient.recipe_id);

        if let Some(ingredient_name) = input_ingredient.ingredient_name {
            new_ingredient.ingredient_name = ingredient_name;
        } else {
            new_ingredient.ingredient_name = old_ingredient.ingredient_name;
        }

        if let Some(ingredient_quantity) = input_ingredient.ingredient_quantity {
            new_ingredient.ingredient_quantity = ingredient_quantity;
        } else {
            new_ingredient.ingredient_quantity = old_ingredient.ingredient_quantity;
        }

        if let Some(quantity_unit) = input_ingredient.quantity_unit {
            new_ingredient.quantity_unit = quantity_unit;
        } else {
            new_ingredient.quantity_unit = old_ingredient.quantity_unit;
        }

        let update_query =
            update_ingredient_query(conn, &new_ingredient).map_err(convert_to_rejection)?;
        return Ok(warp::reply::json(&json!(update_query)));
    }
    return Err(Error::user_error("Cannot update ingredient!", StatusCode::FORBIDDEN).into());
}

/// Backend ingredient delete endpoint function
pub async fn delete_ingredient(
    ingredient: Ingredient,
    claims: Option<UserClaims>,
    db_connection: DbConnection,
) -> Result<impl Reply, Rejection> {
    let mut conn: PooledPgConnection = db_connection.map_err(convert_to_rejection)?;

    let recipe = query_full_recipe(
        &mut conn,
        &UrlRecipeQuery {
            id: Some(ingredient.recipe_id),
            name: None,
        },
    )
    .map_err(convert_to_rejection)?;

    if validate_permission(recipe.recipe.user_id, claims) {
        if delete_ingredient_query(conn, &ingredient).map_err(convert_to_rejection)? == 0 {
            return Err(Error::not_found("Ingredient not found").into());
        }

        return Ok(warp::reply::json(
            &json!({"msg":format!("Ingredient {} deleted",ingredient.ingredient_name)}),
        ));
    }
    return Err(Error::user_error("Cannot delete ingredient!", StatusCode::FORBIDDEN).into());
}
