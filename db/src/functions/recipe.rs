use crate::schema::recipe_ingredient::dsl as ingredient_dsl;
use crate::schema::recipe_step::dsl as step_dsl;
use crate::structs::UrlRecipeQuery;
use crate::{schema::recipe::dsl as recipe_dsl, structs::Ingredient};

use diesel::prelude::*;

use crate::{
    db_pool::{DieselError, PooledPgConnection},
    structs::{FullRecipe, Recipe, Step},
};

use super::user::get_user_name;

pub fn create_recipe_query(
    mut conn: PooledPgConnection,
    incoming_recipe: &Recipe,
) -> Result<Recipe, DieselError> {
    Ok(diesel::insert_into(recipe_dsl::recipe)
        .values(incoming_recipe)
        .get_result::<Recipe>(&mut conn)?)
}

/// Deletes Recipe record based on id
pub fn delete_recipe_query(
    mut conn: PooledPgConnection,
    incoming_recipe: &UrlRecipeQuery,
) -> Result<(), DieselError> {
    diesel::delete(recipe_dsl::recipe)
        .filter(recipe_dsl::id.eq(incoming_recipe.id.unwrap()))
        .execute(&mut conn)?;

    Ok(())
}
/// Returns full recipe with all fields
///
/// # Arguments
///
/// * `conn` -> A pooled Postgres connection
/// * `incoming_query` -> UrlQuery struct with id or name
pub fn query_full_recipe(
    conn: &mut PooledPgConnection,
    incoming_query: &UrlRecipeQuery,
) -> Result<FullRecipe, DieselError> {
    let mut full_recipe = FullRecipe::default();
    // creating a Recipe
    // conditionally set recipe based on URL query
    if let Some(id) = &incoming_query.id {
        let query_recipe: Recipe = recipe_dsl::recipe
            .filter(recipe_dsl::id.eq(id))
            .get_result(conn)?;
        full_recipe.set_recipe(query_recipe);
    } else if let Some(name) = &incoming_query.name {
        let query_recipe: Recipe = recipe_dsl::recipe
            .filter(recipe_dsl::recipe_name.eq(name))
            .get_result(conn)?;
        full_recipe.set_recipe(query_recipe);
    }
    full_recipe.set_owner_name(get_user_name(conn, full_recipe.recipe.user_id.unwrap())?);

    let query_steps: Vec<Step> = step_dsl::recipe_step
        .filter(step_dsl::recipe_id.eq(full_recipe.recipe.id.unwrap()))
        .get_results::<Step>(conn)?;
    full_recipe.set_steps(query_steps);
    let query_ingredients: Vec<Ingredient> = ingredient_dsl::recipe_ingredient
        .filter(ingredient_dsl::recipe_id.eq(full_recipe.recipe.id.unwrap()))
        .get_results::<Ingredient>(conn)?;
    full_recipe.set_ingredients(query_ingredients);

    Ok(full_recipe)
}

/// Returns a list of `Recipe` struct
pub fn fuzzy_query(
    mut conn: PooledPgConnection,
    recipe_name: &String,
) -> Result<Vec<Recipe>, DieselError> {
    Ok(recipe_dsl::recipe
        .filter(recipe_dsl::recipe_name.like(format!("{:}%", recipe_name)))
        .get_results(&mut conn)?)
}

/// Change details about Recipe (name, observations...)
pub fn update_recipe_query(
    mut conn: PooledPgConnection,
    incoming_recipe: &Recipe,
) -> Result<Recipe, DieselError> {
    Ok(diesel::update(recipe_dsl::recipe)
        .filter(recipe_dsl::id.eq(incoming_recipe.id.unwrap()))
        .set(incoming_recipe)
        .get_result(&mut conn)?)
}
