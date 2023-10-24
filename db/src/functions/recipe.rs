use crate::schema::recipe_ingredient::dsl as ingredient_dsl;
use crate::schema::recipe_step::dsl as step_dsl;
use crate::{schema::recipe::dsl as recipe_dsl, structs::Ingredient};

use diesel::prelude::*;

use crate::{
    db_pool::{DieselError, PooledPgConnection},
    structs::{FullRecipe, Recipe, Step},
};

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
    incoming_recipe: &Recipe,
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
/// * `incoming_recipe` -> Recipe struct (maybe i'll use i32 to represent the id)
pub fn query_recipe(
    mut conn: PooledPgConnection,
    incoming_recipe: &Recipe,
) -> Result<FullRecipe, DieselError> {
    let query_recipe: Recipe = recipe_dsl::recipe
        .filter(recipe_dsl::id.eq(&incoming_recipe.id.unwrap()))
        .get_result(&mut conn)?;

    let query_steps: Vec<Step> = step_dsl::recipe_step
        .filter(step_dsl::recipe_id.eq(&incoming_recipe.id.unwrap()))
        .get_results::<Step>(&mut conn)?;
    let query_ingredients: Vec<Ingredient> = ingredient_dsl::recipe_ingredient
        .filter(ingredient_dsl::recipe_id.eq(incoming_recipe.id.unwrap()))
        .get_results::<Ingredient>(&mut conn)?;

    Ok(FullRecipe {
        recipe: query_recipe,
        ingredients: query_ingredients,
        steps: query_steps,
    })
}

/// Returns a list of `Recipe` struct
pub fn fuzzy_query(
    mut conn: PooledPgConnection,
    incoming_recipe: &Recipe,
) -> Result<Vec<Recipe>, DieselError> {
    Ok(recipe_dsl::recipe
        .filter(recipe_dsl::recipe_name.like(format!("{:}%", incoming_recipe.recipe_name)))
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
