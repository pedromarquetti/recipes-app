use crate::db_pool::{DieselError, PooledPgConnection};
use crate::schema::recipe_ingredient::dsl as ingredient_dsl;
use crate::structs::Ingredient;
use diesel::prelude::*;

/// Ingredient DB function responsible for creating an ingredient
pub fn create_ingredient_query(
    mut conn: PooledPgConnection,
    ingredients: &Vec<Ingredient>,
) -> Result<Vec<Ingredient>, DieselError> {
    Ok(diesel::insert_into(ingredient_dsl::recipe_ingredient)
        .values::<&Vec<Ingredient>>(&ingredients)
        .get_results(&mut conn)?)
}

/// Ingredient DB function responsible for deleting an ingredient
pub fn delete_ingredient_query(
    mut conn: PooledPgConnection,
    ingredient: &Ingredient,
) -> Result<usize, DieselError> {
    Ok(diesel::delete(ingredient_dsl::recipe_ingredient)
        .filter(ingredient_dsl::id.eq(ingredient.id.expect("unexpected error!")))
        .execute(&mut conn)?)
}

/// Ingredient DB function responsible for updating an ingredient's details
pub fn update_ingredient_query(
    mut conn: PooledPgConnection,
    ingredients: &Ingredient,
) -> Result<Ingredient, DieselError> {
    Ok(diesel::update(ingredient_dsl::recipe_ingredient)
        .filter(ingredient_dsl::id.eq(ingredients.id.unwrap()))
        .set(ingredients)
        .get_result(&mut conn)?)
}
