use crate::db_pool::{DieselError, PooledPgConnection};
use crate::schema::recipe_ingredient::dsl as ingredient_dsl;
use crate::structs::Ingredient;
use diesel::prelude::*;

/// Ingredient DB function responsible for creating an ingredient
pub fn create_recipe_ingredient_record(
    mut conn: PooledPgConnection,
    ingredients: &Vec<Ingredient>,
) -> Result<(), DieselError> {
    diesel::insert_into(ingredient_dsl::recipe_ingredient)
        .values::<&Vec<Ingredient>>(&ingredients)
        .execute(&mut conn)?;
    Ok(())
}

/// Ingredient DB function responsible for deleting an ingredient
pub fn delete_recipe_ingredient_record(
    mut conn: PooledPgConnection,
    ingredient: &Ingredient,
) -> Result<(), DieselError> {
    diesel::delete(ingredient_dsl::recipe_ingredient)
        .filter(ingredient_dsl::id.eq(ingredient.id.unwrap()))
        .execute(&mut conn)?;
    Ok(())
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
