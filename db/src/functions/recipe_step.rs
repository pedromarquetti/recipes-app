use crate::schema::recipe_step::dsl as step_dsl;
use crate::{
    db_pool::{DieselError, PooledPgConnection},
    structs::Step,
};
use diesel::prelude::*;

/// # DB crate
pub fn create_recipe_step_record(
    mut conn: PooledPgConnection,
    steps: &Vec<Step>,
) -> Result<(), DieselError> {
    diesel::insert_into(step_dsl::recipe_step)
        .values::<&Vec<Step>>(&steps)
        .execute(&mut conn)?;
    Ok(())
}

/// # DB crate
pub fn delete_recipe_step_record(
    mut conn: PooledPgConnection,
    steps: &Step,
) -> Result<(), DieselError> {
    diesel::delete(step_dsl::recipe_step)
        .filter(step_dsl::id.eq(steps.id.unwrap()))
        .execute(&mut conn)?;
    Ok(())
}

/// # DB crate
pub fn update_step_query(mut conn: PooledPgConnection, steps: &Step) -> Result<Step, DieselError> {
    Ok(diesel::update(step_dsl::recipe_step)
        .filter(step_dsl::id.eq(steps.id.unwrap()))
        .set(steps)
        .get_result(&mut conn)?)
}
