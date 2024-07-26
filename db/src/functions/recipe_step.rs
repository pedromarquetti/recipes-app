use crate::structs::NewStep;
use crate::{
    db_pool::{DieselError, PooledPgConnection},
    structs::Step,
};
use diesel::prelude::*;

/// # DB crate
pub fn create_step_query(
    mut conn: PooledPgConnection,
    steps: &Vec<NewStep>,
) -> Result<Vec<Step>, DieselError> {
    use crate::schema::recipe_step::dsl as step_dsl;

    Ok(diesel::insert_into(step_dsl::recipe_step)
        .values(steps)
        .get_results(&mut conn)?)
}

/// # DB crate
pub fn delete_step_query(mut conn: PooledPgConnection, step: &Step) -> Result<usize, DieselError> {
    use crate::schema::recipe_step::dsl as step_dsl;

    Ok(diesel::delete(step_dsl::recipe_step)
        .filter(step_dsl::id.eq(step.id))
        .execute(&mut conn)?)
}

/// # DB crate
pub fn update_step_query(mut conn: PooledPgConnection, steps: &Step) -> Result<Step, DieselError> {
    use crate::schema::recipe_step::dsl as step_dsl;

    Ok(diesel::update(step_dsl::recipe_step)
        .filter(step_dsl::id.eq(steps.id))
        .set(steps)
        .get_result(&mut conn)?)
}
