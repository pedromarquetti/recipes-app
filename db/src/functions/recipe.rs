use crate::structs::{NewRecipe, UrlRecipeQuery};

use diesel::prelude::*;
use diesel::result::DatabaseErrorKind;

use crate::{
    db_pool::{DieselError, PooledPgConnection},
    structs::{FullRecipe, Recipe, Step},
};

use super::user::get_user_name;
pub fn query_recipe(
    conn: &mut PooledPgConnection,
    incoming_recipe: UrlRecipeQuery,
) -> Result<Recipe, DieselError> {
    use crate::schema::recipe::dsl as recipe_dsl;
    if let Some(input_id) = &incoming_recipe.id {
        Ok(recipe_dsl::recipe
            .select((
                recipe_dsl::id,
                recipe_dsl::user_id,
                recipe_dsl::recipe_name,
                recipe_dsl::recipe_observations,
            ))
            .filter(recipe_dsl::id.eq(input_id))
            .first::<Recipe>(conn)?)
    } else if let Some(input_name) = &incoming_recipe.name {
        Ok(recipe_dsl::recipe
            .select((
                recipe_dsl::id,
                recipe_dsl::user_id,
                recipe_dsl::recipe_name,
                recipe_dsl::recipe_observations,
            ))
            .filter(recipe_dsl::recipe_name.eq(input_name))
            .first::<Recipe>(conn)?)
    } else {
        Err(DieselError::DatabaseError(
            DatabaseErrorKind::NotNullViolation,
            Box::new(String::from("Name or ID must be supplied")),
        ))
    }
}

pub fn create_recipe_query(
    conn: &mut PooledPgConnection,
    incoming_recipe: &NewRecipe,
) -> Result<Recipe, DieselError> {
    use crate::schema::recipe::dsl as recipe_dsl;
    diesel::insert_into(recipe_dsl::recipe)
        .values(incoming_recipe.clone())
        .execute(conn)?;
    Ok(query_recipe(
        conn,
        UrlRecipeQuery {
            id: None,
            name: Some(incoming_recipe.recipe_name.clone()),
        },
    )?)
}

/// Deletes Recipe record based on id
pub fn delete_recipe_query(
    conn: &mut PooledPgConnection,
    incoming_recipe: &UrlRecipeQuery,
) -> Result<usize, DieselError> {
    use crate::schema::recipe::dsl as recipe_dsl;
    if let Some(input_id) = &incoming_recipe.id {
        Ok(diesel::delete(recipe_dsl::recipe)
            .filter(recipe_dsl::id.eq(input_id))
            .execute(conn)?)
    } else if let Some(input_name) = &incoming_recipe.name {
        Ok(diesel::delete(recipe_dsl::recipe)
            .filter(recipe_dsl::recipe_name.eq(input_name))
            .execute(conn)?)
    } else {
        Err(DieselError::DatabaseError(
            DatabaseErrorKind::NotNullViolation,
            Box::new(String::from("Name or ID must be supplied")),
        ))
    }
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
    use crate::schema::recipe_ingredient::dsl as ingredient_dsl;
    use crate::schema::recipe_step::dsl as step_dsl;
    use crate::{schema::recipe::dsl as recipe_dsl, structs::Ingredient};
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
    full_recipe.set_owner_name(get_user_name(conn, full_recipe.recipe.user_id)?);

    let query_steps: Vec<Step> = step_dsl::recipe_step
        .filter(step_dsl::recipe_id.eq(full_recipe.recipe.id))
        .get_results::<Step>(conn)?;
    full_recipe.set_steps(query_steps);
    let query_ingredients: Vec<Ingredient> = ingredient_dsl::recipe_ingredient
        .filter(ingredient_dsl::recipe_id.eq(full_recipe.recipe.id))
        .get_results::<Ingredient>(conn)?;
    full_recipe.set_ingredients(query_ingredients);

    Ok(full_recipe)
}

/// Returns a list of `Recipe` struct
pub fn fuzzy_query(
    conn: &mut PooledPgConnection,
    recipe_name: &String,
) -> Result<Vec<Recipe>, DieselError> {
    use crate::schema::recipe::dsl as recipe_dsl;
    Ok(recipe_dsl::recipe
        .filter(recipe_dsl::recipe_name.like(format!("{:}%", recipe_name)))
        .get_results(conn)?)
}

/// Change details about Recipe (name, observations...)
pub fn update_recipe_query(
    conn: &mut PooledPgConnection,
    incoming_recipe: &Recipe,
) -> Result<Recipe, DieselError> {
    use crate::schema::recipe::dsl as recipe_dsl;
    Ok(diesel::update(recipe_dsl::recipe)
        .filter(recipe_dsl::id.eq(incoming_recipe.id))
        .set(incoming_recipe)
        .get_result(conn)?)
}
