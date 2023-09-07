use crate::schema::recipe::dsl as recipe_dsl;
use crate::schema::recipe_step::dsl as step_dsl;

use diesel::prelude::*;

use crate::{
    db_pool::{DieselError, PooledPgConnection},
    structs::{FullRecipe, Recipe, Step},
};

pub fn create_recipe_query(
    mut conn: PooledPgConnection,
    incoming_recipe: &Recipe,
) -> Result<usize, DieselError> {
    let recipe_id = diesel::insert_into(recipe_dsl::recipe)
        .values(incoming_recipe)
        .returning(recipe_dsl::id)
        .execute(&mut conn)?;

    Ok(recipe_id)
}

pub fn delete_recipe_query(
    mut conn: PooledPgConnection,
    incoming_recipe: &Recipe,
) -> Result<(), DieselError> {
    diesel::delete(recipe_dsl::recipe)
        .filter(recipe_dsl::id.eq(incoming_recipe.id.unwrap()))
        .execute(&mut conn)?;

    Ok(())
}

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

    Ok(FullRecipe {
        recipe: query_recipe,
        steps: query_steps,
    })
}
