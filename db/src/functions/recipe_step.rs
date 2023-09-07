use diesel::prelude::*;

use crate::{
    db_pool::{DieselError, PooledPgConnection},
    structs::Step,
};

pub fn create_recipe_step_record(
    mut conn: PooledPgConnection,
    steps: &Vec<Step>,
) -> Result<(), DieselError> {
    use crate::schema::recipe_step;
    diesel::insert_into(recipe_step::table)
        .values::<&Vec<Step>>(&steps)
        .execute(&mut conn)?;
    Ok(())
}
pub fn delete_recipe_step_record(
    mut conn: PooledPgConnection,
    steps: &Step,
) -> Result<(), DieselError> {
    use crate::schema::recipe_step;
    diesel::delete(recipe_step::table)
        .filter(recipe_step::id.eq(steps.id.unwrap()))
        .execute(&mut conn)?;
    Ok(())
}
